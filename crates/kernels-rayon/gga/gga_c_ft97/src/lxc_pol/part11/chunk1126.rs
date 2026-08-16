//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1126/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1126(t10568: f64, t1775: f64, t2739: f64, t505: f64, t11176: f64, t303: f64, t10607: f64, t10362: f64, t289: f64, t287: f64, t2726: f64, t2735: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43563 = t1775 * t10568;
    let t43568 = t505 * t2739;
    let t43574 = 280.0_f64 / 81.0_f64 * t11176 * t303;
    let t43578 = t1775 * t10607;
    let t43585 = 1.0_f64 / t10362 / t289;
    let t43586 = t287 * t43585;
    let t43587 = t2726 * t2726;
    let t43595 = t2735 * t2735;
    (t43563, t43568, t43574, t43578, t43586, t43587, t43595)
}
