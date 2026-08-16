//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 609/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk609(t1909: f64, t8425: f64, t1843: f64, t376: f64, t89: f64, t7822: f64, t7775: f64, t7778: f64, t7748: f64, t7758: f64, t7768: f64, t7791: f64, t7796: f64, t7809: f64, t7813: f64, t7817: f64, t7827: f64, t7831: f64) -> (f64, f64, f64) {
    let t8426 = t1909 * t8425;
    let t8430 = t89 * t376 * t1843;
    let t8437 = 2.0_f64 / 9.0_f64 * t7822;
    let t8443 = 4.0_f64 / 27.0_f64 * t7775;
    let t8444 = t7778 / 9.0_f64;
    let t8445 = 2.0_f64 / 3.0_f64 * t7791 + 2.0_f64 / 9.0_f64 * t7796 - 2.0_f64 / 9.0_f64 * t7809 + t7813 / 3.0_f64 + t7817 / 3.0_f64 - t8437 - 2.0_f64 / 3.0_f64 * t7827 - 2.0_f64 / 3.0_f64 * t7831 - t7748 / 9.0_f64 + 2.0_f64 * t7758 - 10.0_f64 / 81.0_f64 * t7768 - t8443 + t8444;
    (t8426, t8430, t8445)
}
