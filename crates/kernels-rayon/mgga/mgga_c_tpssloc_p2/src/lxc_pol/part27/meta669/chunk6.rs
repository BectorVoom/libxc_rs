//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2369/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2369(t2022: f64, t2319: f64, t1458: f64, t16538: f64, t16541: f64, t2363: f64, t23877: f64, t23880: f64, t26523: f64, t4072: f64, t5376: f64, t577: f64, t671: f64, t83980: f64, t86642: f64, t86646: f64, t86647: f64, t86651: f64, t86653: f64, t86655: f64, t86656: f64, t86660: f64, t86668: f64, t91792: f64, t91799: f64, t91802: f64) -> f64 {
    let t91803 = t2022 * t2319;
    let t91806 = t86642 + 0.135e2_f64 * t26523 * t2363 + t86646 + 27.0_f64 * t86647 * t2319 + t86651 + t86653 + t86655 + 27.0_f64 * t86656 * t671 + t86660 + 27.0_f64 * t23877 * t4072 + 54.0_f64 * t23880 * t16538 + 27.0_f64 * t23880 * t16541 + t86668 + 0.45e1_f64 * t91792 * t577 + 54.0_f64 * t83980 * t5376 + t91799 + t91802 + 27.0_f64 * t91803 * t1458;
    t91806
}
