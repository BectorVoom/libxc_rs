//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1167/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1167<F: Float>(t2328: F, t8021: F, t3157: F, t6117: F, t2340: F, t8028: F, t2380: F, t6475: F, t8474: F, t8345: F, t1220: F, t6377: F, t3235: F, t3237: F, t5939: F, t179: F, t3026: F, t404: F, t6380: F) -> (F, F, F, F, F, F, F, F) {
    let t22406 = 0.70178683471615754484e1 * t2328 * t8021;
    let t22408 = 0.17544670867903938621e1 * t6117 * t3157;
    let t22410 = 0.51947577317044391276e2 * t8028 * t2340;
    let t22445 = t2380 * t6475 * t8474;
    let t22452 = t2380 * t6475 * t8345;
    let t22461 = t1220 * t6377;
    let t22469 = t3235 * t5939 * t3237;
    let t22474 = t404 * t179 * t6380 * t3026;
    (t22406, t22408, t22410, t22445, t22452, t22461, t22469, t22474)
}
