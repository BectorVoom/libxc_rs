//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 617/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk617<F: Float>(t1356: F, t15516: F, t3230: F, t623: F, t2412: F, t3219: F, t1986: F, t2472: F, t675: F, t2471: F, t36: F, t739: F) -> (F, F, F, F, F, F, F) {
    let t15517 = t1356 * t15516;
    let t15518 = F::cast_from(0.39914139006212695214e-1_f64) * t15517;
    let t15519 = t623 * t3230;
    let t15520 = F::cast_from(0.19957069503106347607e-1_f64) * t15519;
    let t15521 = t2412 * t3219;
    let t15522 = F::cast_from(0.42564599893297839398e-5_f64) * t15521;
    let t15523 = t1986 * t2472;
    let t15524 = t675 * t15523;
    let t15525 = F::cast_from(0.42564599893297839398e-5_f64) * t15524;
    let t15526 = t2471 * t36;
    let t15527 = t739 * t15526;
    (t15518, t15520, t15522, t15523, t15525, t15526, t15527)
}
