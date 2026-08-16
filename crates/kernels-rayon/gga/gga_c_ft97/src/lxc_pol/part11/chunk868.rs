//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 868/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk868(t1725: f64, t8109: f64, t2248: f64, t419: f64, t424: f64, t67: f64, t8063: f64, t9: f64, t425: f64, t1732: f64, t8130: f64, t1736: f64, t7763: f64) -> (f64, f64, f64, f64, f64) {
    let t37778 = t1725 * t8109;
    let t37781 = t419 * t2248 * t424;
    let t37784 = t9 * t67 * t8063;
    let t37785 = t37784 * t425;
    let t37787 = t8130 * t1732;
    let t37789 = t1736 * t7763;
    (t37778, t37781, t37785, t37787, t37789)
}
