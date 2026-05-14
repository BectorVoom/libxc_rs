//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 949/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk949<F: Float>(t1356: F, t2211: F, t30311: F, t35478: F, t35481: F, t35484: F, t35487: F, t35514: F, t37731: F, t39901: F, t43179: F, t45864: F, t45866: F, t45869: F, t45872: F, t45874: F, t45880: F, t45884: F, t46005: F, t8041: F, t884: F) -> (F,) {
    let t48684 = 0.162600798888400151e-2 * t35478 - 0.39032073591371545778e-3 * t35481 + 0.162600798888400151e-2 * t35484 - 0.39032073591371545778e-3 * t35487 + t37731 - 0.11974241701863808564e0 * t884 * t2211 * t30311 - 0.11974241701863808564e0 * t1356 * t8041 * t46005 - t43179 + 0.10215503974391481456e-3 * t45864 + 0.1702583995731913576e-4 * t45866 + 0.39726959900411316773e-3 * t39901 - 0.85129199786595678799e-5 * t45869 + 0.66211599834018861287e-4 * t35514 + 0.2553875993597870364e-4 * t45872 - 0.2553875993597870364e-4 * t45874 - 0.2727466165424534173e-1 * t45880 - 0.2727466165424534173e-1 * t45884;
    (t48684,)
}
