//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1236/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1236(t24815: f64, t6252: f64, t24814: f64, t24821: f64, t24820: f64, t5979: f64, t7363: f64, t7362: f64, t5975: f64, t29664: f64, t493: f64, t5971: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29749 = t6252 * t24815;
    let t29750 = t24814 * t29749;
    let t29753 = t6252 * t24821;
    let t29754 = t24820 * t29753;
    let t29758 = t7363 * t5979;
    let t29759 = t7362 * t29758;
    let t29762 = t7363 * t5975;
    let t29763 = t7362 * t29762;
    let t29773 = t493 * t29664;
    let t29776 = t7363 * t5971;
    (t29749, t29750, t29753, t29754, t29758, t29759, t29762, t29763, t29773, t29776)
}
