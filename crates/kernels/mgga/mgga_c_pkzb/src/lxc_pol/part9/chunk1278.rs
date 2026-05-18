//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1278/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1278<F: Float>(t2321: F, t3135: F, t6121: F, t898: F, t2328: F, t8021: F, t3157: F, t6117: F, t2340: F, t8028: F, t2380: F, t6475: F, t8474: F) -> (F, F, F, F, F) {
    let t22404 = F::new(0.31168546390226634765e3) * t898 * t6121 * t3135 * t2321;
    let t22406 = F::new(0.70178683471615754484e1) * t2328 * t8021;
    let t22408 = F::new(0.17544670867903938621e1) * t6117 * t3157;
    let t22410 = F::new(0.51947577317044391276e2) * t8028 * t2340;
    let t22445 = t2380 * t6475 * t8474;
    (t22404, t22406, t22408, t22410, t22445)
}
