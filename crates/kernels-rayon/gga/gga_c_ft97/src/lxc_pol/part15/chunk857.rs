//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 857/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk857(t1736: f64, t7763: f64, t3626: f64, t47: f64, t68: f64, t72: f64, t371: f64, t8052: f64, t19: f64, t7: f64) -> (f64, f64, f64, f64, f64) {
    let t37789 = t1736 * t7763;
    let t37818 = t47 * t3626;
    let t37820 = t68 * t37818 * t72;
    let t37821 = 0.18916624705075445817e-1_f64 * t37820;
    let t37835 = t371 * t8052;
    let t37991 = t7 * t19;
    (t37789, t37820, t37821, t37835, t37991)
}
