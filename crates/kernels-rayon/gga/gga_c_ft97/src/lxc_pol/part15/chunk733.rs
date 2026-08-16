//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 733/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk733(t1039: f64, t4753: f64, t91: f64, t9252: f64, t20564: f64, t9217: f64, t20568: f64, t2102: f64, t20545: f64, t9192: f64, t20362: f64, t2097: f64) -> (f64, f64, f64, f64, f64) {
    let t20782 = t4753 * t1039;
    let t20784 = t91 * t9252 * t20782;
    let t20786 = t9217 * t20564;
    let t20789 = t2102 * t20568;
    let t20793 = t9192 * t20545;
    let t20796 = t2097 * t20362;
    (t20784, t20786, t20789, t20793, t20796)
}
