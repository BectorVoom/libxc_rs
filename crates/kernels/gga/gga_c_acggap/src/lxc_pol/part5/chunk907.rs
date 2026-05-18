//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 907/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk907<F: Float>(t3111: F, t3121: F, t1068: F, t3114: F, t1072: F, t3126: F, t922: F, t3124: F, t839: F, t130: F, t972: F, t1: F, t136: F, t14: F, t195: F, t3: F, t721: F) -> (F, F, F, F, F) {
    let t13701 = t3111 * t3121;
    let t13703 = t1068 * t3114;
    let t13706 = t13703 * t1072 * t922 * t3126;
    let t13714 = t3124 * t1072 * t839 * t3126;
    let t13716 = t130 * t972;
    let t13726 = t13716 * t136 / t14 / t1 / t3 / t195 * t721 / F::new(48.0);
    (t13701, t13703, t13706, t13714, t13726)
}
