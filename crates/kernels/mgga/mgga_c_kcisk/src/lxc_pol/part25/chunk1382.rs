//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1382/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1382<F: Float>(t112858: F, t34533: F, t9740: F, t10487: F, t2029: F, t33207: F, t9990: F, t10014: F, t112765: F, t112872: F, t117068: F, t117074: F, t117078: F, t117084: F, t117934: F, t15909: F, t15921: F, t15930: F, t33208: F, t33225: F, t33229: F, t33287: F, t34412: F, t34424: F, t34496: F, t34499: F, t34560: F, t9743: F) -> (F,) {
    let t118405 = 0.11574074074074074074e-2 * t9740 * t112858 * t34533;
    let t118412 = t2029 * t10487;
    let t118419 = t9990 * t33207;
    let t118428 = -0.13888888888888888889e-1 * t9740 * t117934 * t34499 * t15909 + 0.13402777777777777778e-2 * t112872 * t34496 + 0.13402777777777777778e-2 * t112765 * t34496 + t118405 - 0.92592592592592592594e-2 * t34412 * t33229 + 0.34722222222222222222e-2 * t9740 * t33225 * t34499 * t15921 + 0.13888888888888888889e-1 * t9740 * t34560 * t118412 * t15930 - 0.20833333333333333334e-1 * t33208 * t34424 - 0.34722222222222222222e-2 * t118419 * t9743 + 0.10416666666666666667e-1 * t33287 * t10014 + 0.23214722222222222222e-2 * t117068 - 0.17411041666666666666e-2 * t117074 - 0.23214722222222222222e-2 * t117078 - 0.41270617283950617282e-2 * t117084;
    (t118428,)
}
