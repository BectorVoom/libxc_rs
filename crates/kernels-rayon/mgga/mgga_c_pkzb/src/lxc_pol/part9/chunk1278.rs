//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1278/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1278(t2321: f64, t3135: f64, t6121: f64, t898: f64, t2328: f64, t8021: f64, t3157: f64, t6117: f64, t2340: f64, t8028: f64, t2380: f64, t6475: f64, t8474: f64) -> (f64, f64, f64, f64, f64) {
    let t22404 = 0.31168546390226634765e3_f64 * t898 * t6121 * t3135 * t2321;
    let t22406 = 0.70178683471615754484e1_f64 * t2328 * t8021;
    let t22408 = 0.17544670867903938621e1_f64 * t6117 * t3157;
    let t22410 = 0.51947577317044391276e2_f64 * t8028 * t2340;
    let t22445 = t2380 * t6475 * t8474;
    (t22404, t22406, t22408, t22410, t22445)
}
