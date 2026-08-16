//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 919/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk919(t1443: f64, t2372: f64, t2486: f64, t6154: f64, t1456: f64, t2492: f64, t9802: f64, t6837: f64, t761: f64, t255: f64, t41848: f64, t28128: f64, t53798: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t110401 = t2372 * t1443;
    let t110438 = t2486 * t6154;
    let t110478 = t2492 * t1456;
    let t110539 = t9802 * t1456;
    let t110629 = t761 * t6837;
    let t110660 = t41848 * t255;
    let t110669 = t53798 * t28128;
    (t110401, t110438, t110478, t110539, t110629, t110660, t110669)
}
