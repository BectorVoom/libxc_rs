//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1064/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1064<F: Float>(t27428: F, t4142: F, t1014: F, t27479: F, t2237: F, t54162: F, t7915: F, t7900: F, t27348: F, t27410: F, t18210: F, t27415: F, t7898: F, t11425: F, t1386: F, t94469: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t94474 = t4142 * t27428;
    let t94483 = t1014 * t27479;
    let t94489 = t2237 * t54162 * t7915;
    let t94491 = t54162 * t7900;
    let t94492 = t2237 * t94491;
    let t94494 = t27410 * t27348;
    let t94496 = t18210 * t27415;
    let t94497 = t7898 * t94496;
    let t94499 = t2237 * t94496;
    let t94519 = t1386 * t11425;
    let t94524 = t7898 * t94491;
    let t94526 = t7898 * t94469;
    (t94474, t94483, t94489, t94492, t94494, t94497, t94499, t94519, t94524, t94526)
}
