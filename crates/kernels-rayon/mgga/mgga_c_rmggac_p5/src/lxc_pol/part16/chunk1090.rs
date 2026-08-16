//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1090/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1090(t1356: f64, t2211: f64, t30311: f64, t35478: f64, t35481: f64, t35484: f64, t35487: f64, t35514: f64, t37731: f64, t39901: f64, t43179: f64, t45864: f64, t45866: f64, t45869: f64, t45872: f64, t45874: f64, t45880: f64, t45884: f64, t46005: f64, t8041: f64, t884: f64) -> f64 {
    let t48684 = 0.162600798888400151e-2_f64 * t35478 - 0.39032073591371545778e-3_f64 * t35481 + 0.162600798888400151e-2_f64 * t35484 - 0.39032073591371545778e-3_f64 * t35487 + t37731 - 0.11974241701863808564e0_f64 * t884 * t2211 * t30311 - 0.11974241701863808564e0_f64 * t1356 * t8041 * t46005 - t43179 + 0.10215503974391481456e-3_f64 * t45864 + 0.1702583995731913576e-4_f64 * t45866 + 0.39726959900411316773e-3_f64 * t39901 - 0.85129199786595678799e-5_f64 * t45869 + 0.66211599834018861287e-4_f64 * t35514 + 0.2553875993597870364e-4_f64 * t45872 - 0.2553875993597870364e-4_f64 * t45874 - 0.2727466165424534173e-1_f64 * t45880 - 0.2727466165424534173e-1_f64 * t45884;
    t48684
}
