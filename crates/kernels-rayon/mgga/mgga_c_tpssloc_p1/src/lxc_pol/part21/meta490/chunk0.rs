//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2096/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2096(t828: f64, t9975: f64, t16815: f64, t16758: f64, t4182: f64, t2732: f64, t5617: f64, t829: f64, t1499: f64, t4290: f64, t4166: f64, t4177: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16816 = t9975 * t828;
    let t16817 = t16815 * t16816;
    let t16820 = t16758 * t4182;
    let t16823 = t2732 * t5617;
    let t16825 = t16815 * t4182;
    let t16828 = t16815 * t829;
    let t16830 = t1499 * t4290;
    let t16836 = t4166 * t4177;
    (t16816, t16817, t16820, t16823, t16825, t16828, t16830, t16836)
}
