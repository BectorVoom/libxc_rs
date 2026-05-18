//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1167/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1167<F: Float>(t16901: F, t501: F, t7024: F, t16910: F, t16917: F, t16919: F, t16929: F, t2609: F, t5152: F, t114: F, t557: F, t6798: F) -> (F, F, F, F, F, F, F, F) {
    let t20346 = F::new(0.15584273195113317383e3) * t16901;
    let t20347 = t501 * t7024;
    let t20348 = F::new(12.0) * t20347;
    let t20349 = F::new(8.0) * t16910;
    let t20350 = F::new(0.18311447306006545054e-3) * t16917;
    let t20351 = F::new(0.73245789224026180215e-3) * t16919;
    let t20352 = F::new(960.0) * t16929;
    let t20353 = t2609 * t5152;
    let t20354 = F::new(0.10254018858216406658e4) * t20353;
    let t20356 = t6798 * t114 * t557;
    (t20346, t20348, t20349, t20350, t20351, t20352, t20354, t20356)
}
