//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 969/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk969<F: Float>(t9385: F, t986: F, t995: F, t9274: F, t7192: F, t7195: F, t7214: F, t9271: F, t9292: F, t385: F, t1003: F, t1005: F, t1436: F, t260: F, t2605: F, t2609: F, t2613: F, t2617: F, t3608: F, t3614: F, t3618: F, t3623: F, t7434: F, t9262: F, t9265: F, t9296: F, t9297: F, t9301: F, t9305: F, t9311: F, t9318: F) -> (F, F, F, F, F) {
    let t9387 = t986 * t9385 * t995;
    let t9393 = 0.18541666666666666667e-1 * t9274;
    let t9395 = -t7214 + 0.24722222222222222222e-1 * t7192 - 0.92708333333333333333e-2 * t7195 + 0.12361111111111111111e-1 * t9271 - t9393 + 0.278125e-1 * t9292;
    let t9396 = t9395 * t385;
    let t9399 = -0.10254018858216406658e4 * t1003 * t9262 - 0.35089341735807877242e1 * t1003 * t9265 - t9296 - 0.17315859105681463759e2 * t1003 * t9297 - 0.34631718211362927518e2 * t1003 * t9301 + 0.10389515463408878255e3 * t1003 * t9305 + 0.23392894490538584828e1 * t2605 * t3614 + 0.23392894490538584828e1 * t1003 * t9311 - 0.34631718211362927518e2 * t2605 * t3623 + 0.11696447245269292414e1 * t3608 * t2609 - 0.11696447245269292414e1 * t9318 * t1005 - 0.5848223622634646207e0 * t3608 * t2613 - 0.17315859105681463759e2 * t3608 * t2617 - 0.11696447245269292414e1 * t2605 * t3618 - 0.5848223622634646207e0 * t7434 * t1436 - 0.5848223622634646207e0 * t1003 * t9387 + 0.19751673498613801407e-1 * t260 * t9396;
    (t9387, t9393, t9395, t9396, t9399)
}
