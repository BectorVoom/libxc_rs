//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2480/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2480(t92: f64, t9384: f64, t100: f64, t9398: f64, t2341: f64, t657: f64, t4063: f64, t591: f64, t4053: f64, t1406: f64, t9238: f64, t39031: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45697 = t92 * t9384;
    let t45707 = t100 * t9398;
    let t45717 = t657 * t2341;
    let t45751 = 20.0_f64 * t100 * t4063 * t591;
    let t45762 = 20.0_f64 * t92 * t4053 * t591;
    let t45844 = t1406 * t9238;
    let t45870 = 24.0_f64 * t39031;
    (t45697, t45707, t45717, t45751, t45762, t45844, t45870)
}
