//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1962/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1962(t52: f64, t8027: f64, t2136: f64, t461: f64, t7573: f64, t7324: f64, t3448: f64, t4729: f64, t475: f64, t5011: f64, t68: f64, t7328: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27680 = t8027 * t52;
    let t27681 = t27680 * t2136;
    let t27683 = t7573 * t461;
    let t27684 = t7324 * t27683;
    let t27687 = t3448 * t4729;
    let t27691 = t5011 * t68 * t475;
    let t27692 = t7328 * t27691;
    (t27681, t27683, t27684, t27687, t27691, t27692)
}
