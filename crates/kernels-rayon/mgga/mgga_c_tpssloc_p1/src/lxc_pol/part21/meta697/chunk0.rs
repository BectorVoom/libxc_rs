//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2524/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2524(t10190: f64, t13861: f64, t2986: f64, t13559: f64, t13779: f64, t10189: f64, t4540: f64, t2990: f64, t42771: f64, t4514: f64, t43057: f64, t13913: f64, t2960: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48030 = t2986 * t10190 * t13861;
    let t48044 = t2986 * t13779 * t13559;
    let t48046 = t10189 * t4540;
    let t48048 = t2986 * t48046 * t2990;
    let t48052 = t2986 * t42771 * t4514;
    let t48061 = t2986 * t43057 * t4514;
    let t48063 = t2960 * t13913;
    (t48030, t48044, t48046, t48048, t48052, t48061, t48063)
}
