//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1334/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1334<F: Float>(t1145: F, t2893: F, t4540: F, t2889: F, t4544: F, t11549: F, t26927: F, t2869: F, t2875: F, t2881: F, t2922: F, t2927: F, t31363: F, t31367: F, t4530: F, t4577: F, t7721: F, t7739: F, t7769: F, t7775: F, t9782: F) -> (F,) {
    let t31382 = t1145 * t4540 * t2893;
    let t31386 = t1145 * t4544 * t2889;
    let t31390 = t1145 * t4544 * t2893;
    let t31405 = -90.0 * t7721 * t1145 * t4544 * t2869 + 60.0 * t7775 * t1145 * t4530 * t2889 - 4.0 * t9782 * t11549 - 4.0 * t26927 * t4577 + 21.0 * t2875 * t31390 + 3.0 * t2881 * t31390 - 18.0 * t2922 * t31382 - 18.0 * t2922 * t31386 - 2.0 * t2927 * t31382 - 2.0 * t2927 * t31386 + 6.0 * t7739 * t31363 - 12.0 * t7769 * t31367;
    (t31405,)
}
