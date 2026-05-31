//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 977/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk977<F: Float>(t132: F, t9011: F, t1238: F, t6975: F, t2460: F, t3: F, t1793: F, t675: F, t2002: F, t2028: F, t3463: F, t3466: F, t461: F, t937: F, zeta_threshold: F) -> (F, F, F, F) {
    let t133 = t132 <= zeta_threshold;
    let t9012 = F::cast_from(0.18541666666666666667e-1_f64) * t9011;
    let t9013 = t6975 * t1238;
    let t9016 = t2460 * t3;
    let t9017 = t1793 * t675;
    let t9027 = piecewise3::<F>(t133, F::cast_from(0.0_f64), -F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t9013 * t2028 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t9016 * t9017 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3463 * t2002 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t937 * t1793 - F::cast_from(2.0_f64) * t3466 * t461);
    (t9012, t9013, t9017, t9027)
}
