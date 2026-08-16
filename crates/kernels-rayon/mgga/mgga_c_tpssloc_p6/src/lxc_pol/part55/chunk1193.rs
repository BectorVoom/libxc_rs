//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1193/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1193(t1888: f64, t232: f64, t6646: f64, t87620: f64, t23110: f64, t23185: f64, t32822: f64, t112990: f64, t112995: f64, t113005: f64, t118730: f64, t118735: f64, t118736: f64, t118737: f64, t118739: f64, t118743: f64, t118745: f64, t118751: f64, t118756: f64, t118760: f64, t1499: f64, t2617: f64, t30695: f64, t30726: f64, t32831: f64, t4162: f64, t4166: f64, t8360: f64) -> f64 {
    let t118764 = 0.16449340668482264365e-1_f64 * t1888 * t6646 * t87620 * t232;
    let t118766 = t23185 * t23110 * t32822;
    let t118767 = 0.82246703342411321825e-2_f64 * t118766;
    let t118768 = t1499 * t30726 - t2617 * t32831 - t30695 * t4166 + t4162 * t8360 + t112990 + t112995 - t113005 + t118730 - t118735 - t118736 - t118737 - t118739 + t118743 + t118745 - t118751 - t118756 - t118760 - t118764 + t118767;
    t118768
}
