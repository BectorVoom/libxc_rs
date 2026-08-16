//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1090/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1090<F: Float>(t1356: F, t2211: F, t30311: F, t35478: F, t35481: F, t35484: F, t35487: F, t35514: F, t37731: F, t39901: F, t43179: F, t45864: F, t45866: F, t45869: F, t45872: F, t45874: F, t45880: F, t45884: F, t46005: F, t8041: F, t884: F) -> F {
    let t48684 = F::cast_from(0.162600798888400151e-2_f64) * t35478 - F::cast_from(0.39032073591371545778e-3_f64) * t35481 + F::cast_from(0.162600798888400151e-2_f64) * t35484 - F::cast_from(0.39032073591371545778e-3_f64) * t35487 + t37731 - F::cast_from(0.11974241701863808564e0_f64) * t884 * t2211 * t30311 - F::cast_from(0.11974241701863808564e0_f64) * t1356 * t8041 * t46005 - t43179 + F::cast_from(0.10215503974391481456e-3_f64) * t45864 + F::cast_from(0.1702583995731913576e-4_f64) * t45866 + F::cast_from(0.39726959900411316773e-3_f64) * t39901 - F::cast_from(0.85129199786595678799e-5_f64) * t45869 + F::cast_from(0.66211599834018861287e-4_f64) * t35514 + F::cast_from(0.2553875993597870364e-4_f64) * t45872 - F::cast_from(0.2553875993597870364e-4_f64) * t45874 - F::cast_from(0.2727466165424534173e-1_f64) * t45880 - F::cast_from(0.2727466165424534173e-1_f64) * t45884;
    t48684
}
