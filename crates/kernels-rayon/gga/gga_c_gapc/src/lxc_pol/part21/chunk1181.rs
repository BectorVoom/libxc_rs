//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1181/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1181(t1030: f64, t11428: f64, t11591: f64, t1461: f64, t505: f64, t11439: f64, t129: f64, t19670: f64, t34605: f64, t34608: f64, t34611: f64, t34613: f64, t34615: f64, t34617: f64, t34619: f64, t34622: f64, t34625: f64) -> f64 {
    let t34630 = t1030 * t1461 * t11428 * t505 * t11591;
    let t34633 = t19670 * t129 * t11439;
    let t34635 = -0.90579542097823505428e-7_f64 * t34605 + 0.18115908419564701086e-6_f64 * t34608 + 0.50589159825786619273e-8_f64 * t34611 - 0.27507855655271474229e-6_f64 * t34613 + 0.20241536458333333334e-3_f64 * t34615 + 0.40483072916666666668e-4_f64 * t34617 - 0.57970906942607043474e-5_f64 * t34619 - 0.17376185052903442709e-3_f64 * t34622 - 0.15445497824803060186e-4_f64 * t34625 + 0.96684272530105650818e-8_f64 * t34630 - 0.21720231316129303386e-4_f64 * t34633;
    t34635
}
