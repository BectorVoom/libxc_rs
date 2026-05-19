//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 428/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk428<F: Float>(t377: F, t4287: F, t364: F, t1076: F, t163: F, t158: F, t1080: F, t4221: F, t4130: F, t4133: F, t4136: F, t4138: F, t4142: F, t4144: F, t4146: F, t4149: F) -> (F, F, F, F) {
    let t4288 = t4287 * t377;
    let t4290 = F::new(1.0) * t364 * t4288;
    let t4292 = F::new(1.0) / t1076 / t163;
    let t4293 = t158 * t4292;
    let t4294 = t4221 * t1080;
    let t4305 = -F::new(0.47063e1) * t4130 + F::cast_from(0.31375333333333333334e1_f64) * t4133 - F::cast_from(0.36604555555555555556e1_f64) * t4136 - F::cast_from(0.16068111111111111111e1_f64) * t4138 + F::cast_from(0.28051666666666666666e0_f64) * t4142 - F::cast_from(0.56103333333333333332e0_f64) * t4144 - F::cast_from(0.6545388888888888889e0_f64) * t4146 - F::cast_from(0.46308888888888888888e0_f64) * t4149;
    (t4290, t4293, t4294, t4305)
}
