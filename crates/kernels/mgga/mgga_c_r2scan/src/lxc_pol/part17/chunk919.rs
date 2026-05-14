//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 919/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk919<F: Float>(t12720: F, t12086: F, t3579: F, t12570: F, t3262: F, t3465: F, t12567: F, t3469: F, t12422: F, t11189: F, t12574: F, t3275: F, t1039: F, t3787: F, t12098: F, t3582: F) -> (F, F, F, F, F, F, F, F) {
    let t12721 = 5.0 / 16.0 * t12720;
    let t12722 = t3579 * t12086;
    let t12723 = t12722 / 2.0;
    let t12725 = t3262 * t3465 * t12570;
    let t12726 = 3.0 / 4.0 * t12725;
    let t12727 = t12567 * t3469;
    let t12728 = t12727 / 4.0;
    let t12729 = t12422 * t3469;
    let t12730 = t12729 / 4.0;
    let t12732 = t3275 * t11189 * t12574;
    let t12733 = 45.0 / 64.0 * t12732;
    let t12734 = t1039 * t3787;
    let t12735 = 2.0 * t12734;
    let t12737 = t3275 * t12098 * t3582;
    (t12721, t12723, t12726, t12728, t12730, t12733, t12735, t12737)
}
