//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta306 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1332;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta306(t3051: f64, t820: f64, t1005: f64, t3082: f64, t121: f64, t3061: f64, t1008: f64, t349: f64, t1011: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t10422, t10436, t10457, t10469, t10470, t10471) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1332(t3051, t820, t1005, t3082, t121, t3061, t1008, t349, t1011);
    (t10422, t10436, t10457, t10469, t10470, t10471)
}
