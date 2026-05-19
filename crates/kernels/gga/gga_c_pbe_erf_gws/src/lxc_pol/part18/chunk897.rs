//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 897/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk897<F: Float>(t50: F, t3351: F, t4767: F, t1412: F, t3354: F, t1351: F, t2465: F, t422: F, t52: F, t9801: F, t59: F, t9992: F, t85: F, zeta_threshold: F) -> (F, F) {
    let t51 = t50 <= zeta_threshold;
    let t9993 = t4767 * t3351;
    let t9998 = t1412 * t3354;
    let t10004 = piecewise3::<F>(t51, F::new(0.0), -F::new(8.0) / F::new(27.0) * t9993 * t422 - F::new(16.0) / F::new(9.0) * t2465 * t1351 + F::new(4.0) / F::new(9.0) * t9998 * t422 + F::new(4.0) / F::new(3.0) * t52 * t9801);
    let t10006 = (t9992 + t10004) * t59;
    let t10007 = t10006 * t85;
    (t10006, t10007)
}
