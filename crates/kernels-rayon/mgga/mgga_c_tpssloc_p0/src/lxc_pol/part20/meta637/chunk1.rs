//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2343/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2343(t2986: f64, t2990: f64, t48046: f64, t42771: f64, t4514: f64, t43057: f64, t13913: f64, t2960: f64, t4542: f64, t698: f64, t973: f64, t10186: f64, t13788: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48048 = t2986 * t48046 * t2990;
    let t48052 = t2986 * t42771 * t4514;
    let t48061 = t2986 * t43057 * t4514;
    let t48063 = t2960 * t13913;
    let t48066 = t973 * t698 * t4542;
    let t48067 = 0.55555555555555555554e-3_f64 * t48066;
    let t48068 = t10186 * t13788;
    (t48048, t48052, t48061, t48063, t48067, t48068)
}
