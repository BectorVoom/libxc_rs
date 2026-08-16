//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2057/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2057(t207: f64, t40419: f64, t9538: f64, t41083: f64, t789: f64, t154: f64, t1891: f64, t205: f64, t792: f64, t9558: f64, t40394: f64, t40399: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41155 = 0.26851851851851851851e-2_f64 * t40419 * t207 * t9538;
    let t41156 = t41083 * t789;
    let t41160 = t154 * t1891;
    let t41161 = t205 * t41160;
    let t41170 = t792 * t9558;
    let t41185 = 0.69444444444444444445e-4_f64 * t40394 * t207 * t40399;
    (t41155, t41156, t41160, t41161, t41170, t41185)
}
