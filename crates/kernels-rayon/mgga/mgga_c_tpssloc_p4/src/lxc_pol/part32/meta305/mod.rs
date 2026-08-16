//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta305 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1330;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1331;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta305(t340: f64, t63: f64, t344: f64, t221: f64, t339: f64, t1032: f64, t3082: f64, t2393: f64, t374: f64, t376: f64, t370: f64, t3158: f64, t964: f64, t3069: f64, t3180: f64, t3036: f64, t67: f64, t3067: f64, t3186: f64, t3062: f64, t820: f64, t3200: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10335, t10339, t10372, t10377, t10381) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1330(t340, t63, t344, t221, t339, t1032, t3082, t2393, t374, t376, t370, t3158, t964);
        let (t10385, t10390, t10401, t10403, t10408, t10413) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1331(t10335, t221, t339, t3069, t3180, t3036, t67, t3067, t3186, t3062, t820, t3200);
    (t10339, t10372, t10377, t10381, t10385, t10390, t10401, t10403, t10408, t10413)
}
