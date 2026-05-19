//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1184/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1184<F: Float>(t1030: F, t11428: F, t11591: F, t1461: F, t505: F, t11439: F, t129: F, t19670: F, t34605: F, t34608: F, t34611: F, t34613: F, t34615: F, t34617: F, t34619: F, t34622: F, t34625: F) -> F {
    let t34630 = t1030 * t1461 * t11428 * t505 * t11591;
    let t34633 = t19670 * t129 * t11439;
    let t34635 = -F::cast_from(0.90579542097823505428e-7_f64) * t34605 + F::cast_from(0.18115908419564701086e-6_f64) * t34608 + F::cast_from(0.50589159825786619273e-8_f64) * t34611 - F::cast_from(0.27507855655271474229e-6_f64) * t34613 + F::cast_from(0.20241536458333333334e-3_f64) * t34615 + F::cast_from(0.40483072916666666668e-4_f64) * t34617 - F::cast_from(0.57970906942607043474e-5_f64) * t34619 - F::cast_from(0.17376185052903442709e-3_f64) * t34622 - F::cast_from(0.15445497824803060186e-4_f64) * t34625 + F::cast_from(0.96684272530105650818e-8_f64) * t34630 - F::cast_from(0.21720231316129303386e-4_f64) * t34633;
    t34635
}
