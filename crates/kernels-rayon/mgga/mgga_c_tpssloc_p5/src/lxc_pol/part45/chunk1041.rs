//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1041/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1041(t31019: f64, t31688: f64, t2240: f64, t240: f64, t8301: f64, t8515: f64, t113824: f64, t113833: f64, t113890: f64, t113907: f64, t115829: f64, t115834: f64, t115837: f64, t115842: f64, t115846: f64, t31672: f64, t31675: f64, t39049: f64, t8511: f64, t8512: f64) -> f64 {
    let t115853 = t31688 * t31019;
    let t115860 = 55.0_f64 / 81.0_f64 * t2240 * t8301 * t240 * t8515;
    let t115861 = 5.0_f64 / 6.0_f64 * t31675 * t113890 + 5.0_f64 / 12.0_f64 * t31675 * t115829 - 5.0_f64 / 9.0_f64 * t113824 * t115834 + 20.0_f64 / 27.0_f64 * t115837 - 5.0_f64 / 18.0_f64 * t8512 * t113907 - 5.0_f64 / 36.0_f64 * t8512 * t115842 + 10.0_f64 / 27.0_f64 * t115846 - 5.0_f64 / 72.0_f64 * t39049 * t8511 * t8515 - 5.0_f64 / 36.0_f64 * t31672 * t31019 + 10.0_f64 / 27.0_f64 * t115853 - 5.0_f64 / 72.0_f64 * t8512 * t113833 - t115860;
    t115861
}
