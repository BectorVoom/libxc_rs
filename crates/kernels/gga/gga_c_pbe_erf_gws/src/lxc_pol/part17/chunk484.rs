//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 484/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk484<F: Float>(t43: F, t50: F, t310: F, t311: F, t1: F, t305: F, t152: F, t6: F, t279: F, t837: F, t1524: F, t1526: F, t1529: F, t1531: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t2057 = F::cast_from(1.0_f64) / t311 / t310;
    let t2059 = t305 * t2057 * t1;
    let t2060 = t152 * t6;
    let t2062 = t2060 * t837 * t279;
    let t2063 = t2059 * t2062;
    let t2064 = F::cast_from(0.63272429661648472106e0_f64) * t2063;
    let t2068 = piecewise3::<F>(t44, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1524 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1526);
    let t2072 = piecewise3::<F>(t51, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1529 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1531);
    (t2057, t2059, t2060, t2062, t2064, t2068, t2072)
}
