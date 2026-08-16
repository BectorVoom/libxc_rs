//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta419 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1530;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta419(t20118: f64, t20147: f64, t3: f64, t112: f64, t6470: f64, t576: f64, t671: f64, t1458: f64, t4072: f64, t5493: f64, t12524: f64, t1401: f64, t16521: f64, t16524: f64, t19534: f64, t3938: f64, t3941: f64, t5371: f64, t5376: f64, t5456: f64, t577: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t20148, t20149, t20162, t20173, t20176, t20181, t20186) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1530(t20118, t20147, t3, t112, t6470, t576, t671, t1458, t4072, t5493, t12524, t1401, t16521, t16524, t19534, t3938, t3941, t5371, t5376, t5456, t577);
    (t20148, t20149, t20162, t20173, t20176, t20181, t20186)
}
