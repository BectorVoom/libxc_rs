//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1309/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1309<F: Float>(t12351: F, t2811: F, t4816: F, t654: F, t10872: F, t1791: F, t32935: F, t5014: F, t10879: F, t9664: F, t9666: F, t44407: F, t662: F, t18325: F, t32941: F, t32947: F) -> (F, F, F, F, F, F, F, F) {
    let t112173 = t2811 * t12351;
    let t112176 = t4816 * t654;
    let t112184 = t10872 * t1791;
    let t112192 = t5014 * t32935;
    let t112212 = t9664 * t10879 * t9666;
    let t112221 = t662 * t44407;
    let t112266 = t32941 * t18325;
    let t112269 = t32947 * t18325;
    (t112173, t112176, t112184, t112192, t112212, t112221, t112266, t112269)
}
