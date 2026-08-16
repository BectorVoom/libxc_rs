//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1978/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1978(t20038: f64, t225: f64, t20032: f64, t20040: f64, t19635: f64, t20048: f64, t1351: f64, t6414: f64, t6387: f64, t6330: f64, t12250: f64, t1834: f64, t5286: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t56434 = t20038 * t225;
    let t56580 = t20032 * t225;
    let t56596 = t20040 * t225;
    let t56607 = t19635 * t225;
    let t56640 = t20048 * t225;
    let t56812 = t6414 * t1351;
    let t57091 = t6387 * t1351;
    let t57172 = t6330 * t1351;
    let t57342 = t6387 * t12250;
    let t57499 = t1834 * t5286;
    (t56434, t56580, t56596, t56607, t56640, t56812, t57091, t57172, t57342, t57499)
}
