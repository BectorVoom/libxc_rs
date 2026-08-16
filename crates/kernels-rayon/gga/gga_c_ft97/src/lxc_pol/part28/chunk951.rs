//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 951/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk951(t1293: f64, t22794: f64, t39: f64, t92353: f64, t136825: f64, t32169: f64, t32170: f64, t136635: f64, t64: f64, t136637: f64, t70: f64, t1546: f64, t7204: f64) -> (f64, f64, f64, f64, f64) {
    let t136935 = t92353 * t1293 * t39 * t22794;
    let t136952 = t32169 * t136825 * t32170;
    let t136967 = t64 * t136635;
    let t136968 = t136637 * t70;
    let t136986 = t7204 * t1546;
    (t136935, t136952, t136967, t136968, t136986)
}
