//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 884/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk884<F: Float>(t43: F, t476: F, t9788: F, t9779: F, t9781: F, t9784: F, t3351: F, t4366: F, t422: F, t1351: F, t2485: F, t1528: F, t3354: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t9789 = t476 * t9788;
    let t9792 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9779 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t9781 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9784 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t9789);
    let t9793 = t4366 * t3351;
    let t9794 = t9793 * t422;
    let t9796 = t2485 * t1351;
    let t9798 = t1528 * t3354;
    (t9789, t9792, t9794, t9796, t9798)
}
