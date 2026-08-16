//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1308/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1308(t2845: f64, t34303: f64, t34308: f64, t34313: f64, t35369: f64, t35375: f64, t35378: f64, t35714: f64, t36055: f64, t36058: f64, t36067: f64, t36072: f64, t36074: f64, t36078: f64, t36080: f64, t3797: f64) -> f64 {
    let t38848 = t2845 * t3797 - t34303 + t34308 + t34313 - t35369 - t35375 + t35378 + t35714 - t36055 + t36058 - t36067 + t36072 + t36074 + t36078 - t36080;
    t38848
}
