//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1758/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1758(t1184: f64, t52: f64, t460: f64, t24682: f64, t3548: f64, t7310: f64, t2127: f64, t3545: f64, t2132: f64, t607: f64, t2136: f64, t3535: f64, t7338: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24683 = t52 * t1184;
    let t24684 = t24683 * t460;
    let t24685 = t24682 * t24684;
    let t24690 = t7310 * t3548;
    let t24704 = t2127 * t3545 / 432.0_f64;
    let t24711 = t2132 * t607;
    let t24712 = t24711 * t2136;
    let t24716 = t3535 * t7338;
    (t24683, t24684, t24685, t24690, t24704, t24712, t24716)
}
