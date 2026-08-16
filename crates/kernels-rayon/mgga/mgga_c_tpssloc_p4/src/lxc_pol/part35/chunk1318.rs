//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1318/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1318(t252: f64, t5527: f64, t28333: f64, t6562: f64, t794: f64, t22893: f64, t23164: f64, t28345: f64, t28329: f64, t23185: f64, t28426: f64, t81914: f64) -> (f64, f64, f64, f64, f64) {
    let t98336 = t252 * t5527;
    let t98342 = t6562 * t794 * t28333;
    let t98345 = t23164 * t22893 * t28345;
    let t98356 = t23164 * t22893 * t28329;
    let t98363 = t23185 * t81914 * t28426;
    (t98336, t98342, t98345, t98356, t98363)
}
