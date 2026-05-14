//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1029/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1029<F: Float>(t34605: F, t34608: F, t34611: F, t34613: F, t34615: F, t34617: F, t34619: F, t34622: F, t34625: F, t34630: F, t34633: F, t11438: F, t26331: F, t5549: F, t1030: F, t33307: F, t4979: F) -> (F, F, F) {
    let t34635 = -0.90579542097823505428e-7 * t34605 + 0.18115908419564701086e-6 * t34608 + 0.50589159825786619273e-8 * t34611 - 0.27507855655271474229e-6 * t34613 + 0.20241536458333333334e-3 * t34615 + 0.40483072916666666668e-4 * t34617 - 0.57970906942607043474e-5 * t34619 - 0.17376185052903442709e-3 * t34622 - 0.15445497824803060186e-4 * t34625 + 0.96684272530105650818e-8 * t34630 - 0.21720231316129303386e-4 * t34633;
    let t34638 = t11438 * t26331 * t5549;
    let t34641 = t1030 * t33307 * t4979;
    (t34635, t34638, t34641)
}
