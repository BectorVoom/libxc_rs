//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 885/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk885(t1877: f64, t193: f64, t202: f64, t2522: f64, t32029: f64, t32034: f64, t32047: f64, t7109: f64, t7114: f64, t776: f64, t868: f64, t870: f64, t8744: f64, t8748: f64) -> f64 {
    let t32071 = t193 * t202 * t32029 * t870 - t1877 * t32034 * t868 + 2.0_f64 * t1877 * t32047 * t868 - 2.0_f64 * t1877 * t7109 * t7114 + 3.0_f64 * t2522 * t776 * t8744 - 3.0_f64 * t2522 * t776 * t8748;
    t32071
}
