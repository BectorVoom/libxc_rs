//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1113/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1113(t10298: f64, t607: f64, t2242: f64, t2259: f64, t11061: f64, t30: f64, t27383: f64, t50066: f64, t25207: f64, t51775: f64, t41161: f64, t51792: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92709 = t10298 * t607;
    let t92711 = t2242 * t2259;
    let t92743 = t30 * t11061;
    let t92747 = t27383 * t50066;
    let t92753 = t25207 * t51775;
    let t92759 = t25207 * t41161;
    let t92762 = t27383 * t51792;
    (t92709, t92711, t92743, t92747, t92753, t92759, t92762)
}
