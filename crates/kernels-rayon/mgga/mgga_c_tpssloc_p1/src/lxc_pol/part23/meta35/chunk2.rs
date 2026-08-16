//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 252/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk252(t118: f64, t168: f64, t181: f64, t677: f64, t680: f64, t705: f64, t725: f64, t732: f64, t740: f64, t747: f64) -> f64 {
    let t750 = 0.53237641966666666666e-3_f64 * t118 * t677 * t168 + 1.0_f64 * t725 * t732 - t680 - t705 + 0.18311447306006545054e-3_f64 * t118 * t677 * t181 + 0.5848223622634646207e0_f64 * t740 * t747;
    t750
}
