//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1855/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1855<F: Float>(t91154: F, t91158: F, t91161: F, t91170: F, t91214: F, t91225: F, t91281: F, t91283: F, t91286: F, t91290: F, t91300: F, t91303: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t93651 = F::cast_from(0.13457585364713463618e-3_f64) * t91154;
    let t93652 = F::cast_from(0.26915170729426927236e-3_f64) * t91158;
    let t93653 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t91161;
    let t93657 = F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t91170;
    let t93674 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t91214;
    let t93682 = F::cast_from(0.56521858531796547194e-2_f64) * t91225;
    let t93710 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t91281;
    let t93711 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t91283;
    let t93712 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t91286;
    let t93715 = F::cast_from(0.33913115119077928316e-1_f64) * t91290;
    let t93718 = F::cast_from(0.11304371706359309439e-1_f64) * t91300;
    let t93720 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t91303;
    (t93651, t93652, t93653, t93657, t93674, t93682, t93710, t93711, t93712, t93715, t93718, t93720)
}
