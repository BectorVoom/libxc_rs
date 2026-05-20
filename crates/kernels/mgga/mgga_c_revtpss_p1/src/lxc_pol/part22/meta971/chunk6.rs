//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3250/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3250<F: Float>(t18437: F, t2652: F, t2661: F, t2662: F, t4352: F, t4424: F, t18413: F, t837: F, t10716: F, t18402: F, t10722: F, t5993: F) -> (F, F, F, F, F) {
    let t61660 = t2652 * t18437;
    let t61669 = t2661 * t2662 * t4352 * t4424;
    let t61673 = t2661 * t2662 * t18413 * t837;
    let t61675 = t10716 * t18402;
    let t61677 = t10722 * t5993;
    (t61660, t61669, t61673, t61675, t61677)
}
