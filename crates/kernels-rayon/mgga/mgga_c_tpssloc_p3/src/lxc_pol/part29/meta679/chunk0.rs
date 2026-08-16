//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2276/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2276(t11153: f64, t497: f64, t27654: f64, t491: f64, t1235: f64, t8034: f64, t27434: f64, t85639: f64, t27821: f64, t24600: f64, t7301: f64, t27798: f64, t4935: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94349 = t497 * t11153;
    let t94354 = t27654 * t491;
    let t94358 = t8034 * t1235;
    let t94363 = 0.18277045187202515961e-2_f64 * t85639 * t27434;
    let t94365 = 0.18277045187202515961e-2_f64 * t85639 * t27821;
    let t94369 = t24600 * t7301;
    let t94374 = t4935 * t27798;
    (t94349, t94354, t94358, t94363, t94365, t94369, t94374)
}
