//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 878/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk878(t683: f64, t7514: f64, t191: f64, t33300: f64, t2371: f64, t2404: f64, t27: f64, t41751: f64, t241: f64, t41536: f64, t10: f64, t11175: f64, t242: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41825 = t683 * t7514;
    let t41848 = t191 * t33300;
    let t41879 = t2404 * t2371;
    let t41911 = t27 * t41751;
    let t41912 = t241 * t41536;
    let t41950 = t10 * t11175 * t242;
    (t41825, t41848, t41879, t41911, t41912, t41950)
}
