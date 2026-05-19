//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1015/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1015<F: Float>(t11339: F, t326: F, t826: F, t2365: F, t3747: F, t1114: F, t833: F, t1115: F, t2397: F, t2401: F, t3207: F, t335: F, t3913: F, t4487: F, t844: F, t8740: F, t8745: F, t8747: F, t8751: F, t9948: F, t9953: F, t9956: F, t9958: F, t9962: F, t9965: F, t9969: F, t9973: F, t9978: F, param_a_c: F) -> (F, F, F, F) {
    let t11340 = param_a_c * t11339;
    let t11341 = t326 * t11340;
    let t11342 = t11341 * t826;
    let t11347 = t3747 * t2365;
    let t11348 = t1114 * t11347;
    let t11349 = t11348 * t833;
    let t11351 = t3207 * t9948 / F::new(8.0) + t3913 * t2397 / F::new(96.0) - F::new(7.0) / F::new(72.0) * t9953 - t8740 - F::new(7.0) / F::new(288.0) * t9956 + t9958 * t833 / F::new(96.0) + F::new(35.0) / F::new(432.0) * t4487 - F::new(7.0) / F::new(288.0) * t9962 - t335 * t9965 / F::new(96.0) - t844 * t9969 / F::new(48.0) + t2401 * t9973 / F::new(16.0) - t8745 + F::new(35.0) / F::new(216.0) * t8747 - t335 * t9978 / F::new(96.0) + t11342 * t833 / F::new(96.0) + t1115 * t8751 / F::new(48.0) - F::new(7.0) / F::new(288.0) * t11349;
    (t11340, t11342, t11348, t11351)
}
