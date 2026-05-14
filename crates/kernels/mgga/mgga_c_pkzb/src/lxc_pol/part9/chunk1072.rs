//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1072/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1072<F: Float>(t16901: F, t501: F, t7024: F, t16910: F, t16917: F, t16919: F, t16929: F, t2609: F, t5152: F, t114: F, t557: F, t6798: F, t1499: F, t7035: F, t16931: F, t16783: F, t16787: F, t16906: F, t16909: F, t16915: F, t16923: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t20346 = 0.15584273195113317383e3 * t16901;
    let t20347 = t501 * t7024;
    let t20348 = 12.0 * t20347;
    let t20349 = 8.0 * t16910;
    let t20350 = 0.18311447306006545054e-3 * t16917;
    let t20351 = 0.73245789224026180215e-3 * t16919;
    let t20352 = 960.0 * t16929;
    let t20353 = t2609 * t5152;
    let t20354 = 0.10254018858216406658e4 * t20353;
    let t20356 = t6798 * t114 * t557;
    let t20357 = 0.17544670867903938621e1 * t20356;
    let t20358 = t7035 * t1499;
    let t20359 = 0.17544670867903938621e1 * t20358;
    let t20360 = 48.0 * t16931;
    let t20361 = t16783 - t16787 - t20346 - t16906 + t16909 - t20348 + t20349 + t16915 - t20350 + t20351 - t16923 - t20352 - t20354 - t20357 - t20359 - t20360;
    (t20346, t20348, t20349, t20350, t20351, t20352, t20354, t20357, t20359, t20360, t20361)
}
