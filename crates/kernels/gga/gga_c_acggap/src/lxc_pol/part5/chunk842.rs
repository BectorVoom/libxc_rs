//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 842/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk842<F: Float>(t136: F, t357: F, t7599: F, t1074: F, t7309: F, t1059: F, t2015: F, t1062: F, t2035: F, t3127: F, t3111: F, t3121: F, t1068: F, t3114: F, t1072: F, t3126: F, t922: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13690 = t7599 * t136 * t357;
    let t13691 = 0.57050000000000000002e1 * t13690;
    let t13693 = t7309 * t136 * t1074;
    let t13694 = 0.70633333333333333334e1 * t13693;
    let t13695 = t2015 * t1059;
    let t13696 = t13695 * t1062;
    let t13698 = t2035 * t1059;
    let t13699 = t13698 * t3127;
    let t13701 = t3111 * t3121;
    let t13703 = t1068 * t3114;
    let t13706 = t13703 * t1072 * t922 * t3126;
    (t13690, t13691, t13693, t13694, t13695, t13696, t13698, t13699, t13701, t13703, t13706)
}
