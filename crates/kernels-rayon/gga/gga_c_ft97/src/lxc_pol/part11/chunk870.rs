//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 870/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk870(t1725: f64, t8112: f64, t1675: f64, t625: f64, t68: f64, t72: f64, t2247: f64, t391: f64, t3626: f64, t47: f64, t14: f64, t37678: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37808 = t1725 * t8112;
    let t37812 = t68 * t1675 * t625 * t72;
    let t37814 = t391 * t2247;
    let t37816 = t68 * t37814 * t72;
    let t37818 = t47 * t3626;
    let t37820 = t68 * t37818 * t72;
    let t37821 = 0.18916624705075445817e-1_f64 * t37820;
    let t37824 = t68 * t37678 * t14 * t72;
    (t37808, t37812, t37816, t37820, t37821, t37824)
}
