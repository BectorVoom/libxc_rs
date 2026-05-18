//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 911/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk911<F: Float>(t101: F, t10179: F, t10029: F, t10035: F, t10037: F, t10046: F, t2857: F, t2922: F, t2986: F, t3642: F, t481: F, t526: F, t8267: F, t8270: F, t8275: F, t8277: F, t8281: F, t8290: F, t8293: F, t8302: F, t8318: F, t8497: F) -> F {
    let t10180 = t101 * t10179;
    let t10185 = -F::new(0.54045904796391420712e-1) * t10029 + F::new(6.0) * t2986 * t8302 - F::new(0.29056741517886919367e-3) * t10035 + F::new(6.0) * t2857 * t10037 * t481 + t2922 * t3642 - F::new(6.0) * t8497 * t8293 + F::new(6.0) * t2986 * t8318 + F::new(12.0) * t2857 * t10046 + t10180 * t526 - t8267 - F::new(0.23948468020509218188e-1) * t8270 - t8275 - F::new(0.11622696607154767747e-2) * t8277 + F::new(0.27119625416694458076e-2) * t8281 - t8290;
    t10185
}
