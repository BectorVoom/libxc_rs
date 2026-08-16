//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2043/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2043(t16160: f64, t16161: f64, t16163: f64, t16173: f64, t225: f64, t1345: f64, t68: f64, t1799: f64, t1995: f64, t3734: f64, t1365: f64, t5187: f64) -> (f64, f64, f64, f64, f64) {
    let t16176 = (t16160 + t16161 + t16163 + t16173) * t225;
    let t16186 = t1345 * t68;
    let t16191 = t1995 * t1799;
    let t16192 = t16191 * t3734;
    let t16195 = t1365 * t5187;
    (t16176, t16186, t16191, t16192, t16195)
}
