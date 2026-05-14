//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1112/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1112<F: Float>(t18414: F, t2661: F, t93082: F, t18418: F, t25227: F, t18402: F, t25234: F, t18409: F, t25266: F, t5980: F, t18531: F, t25245: F, t18432: F, t93025: F, t18440: F, t18348: F, t1945: F, t807: F) -> (F, F, F, F, F, F, F, F, F) {
    let t106030 = t2661 * t93082 * t18414;
    let t106033 = t2661 * t25227 * t18418;
    let t106037 = t25234 * t18402;
    let t106040 = t2661 * t25227 * t18409;
    let t106042 = t25266 * t5980;
    let t106048 = t25245 * t18531;
    let t106050 = t93025 * t18432;
    let t106053 = t2661 * t25227 * t18440;
    let t106061 = t807 * t1945 * t18348;
    (t106030, t106033, t106037, t106040, t106042, t106048, t106050, t106053, t106061)
}
