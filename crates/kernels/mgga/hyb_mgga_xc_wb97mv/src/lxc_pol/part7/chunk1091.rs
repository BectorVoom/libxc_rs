//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1091/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1091<F: Float>(t4393: F, t996: F, t3597: F, t3613: F, t2572: F, t4372: F, t4359: F, t7258: F, t3622: F, t2594: F, t1003: F, t11474: F, t11477: F, t11480: F, t11483: F, t11486: F, t11489: F, t11493: F, t11496: F, t11500: F, t1436: F, t2605: F, t3608: F, t3614: F, t3623: F, t4386: F, t9318: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11551 = t4393 * t996;
    let t11554 = t3613 * t3597;
    let t11559 = t2572 * t4372;
    let t11560 = t11559 * t996;
    let t11563 = t7258 * t4359;
    let t11564 = t11563 * t3622;
    let t11567 = t2594 * t4372;
    let t11568 = t11567 * t3622;
    let t11571 = -0.11696447245269292414e1 * t9318 * t1436 + 0.11696447245269292414e1 * t2605 * t4386 + 0.23392894490538584828e1 * t3608 * t3614 - 0.35089341735807877242e1 * t1003 * t11551 + 0.23392894490538584828e1 * t1003 * t11554 - 0.34631718211362927517e2 * t3608 * t3623 + 0.11696447245269292414e1 * t1003 * t11560 + 0.10389515463408878255e3 * t1003 * t11564 - 0.17315859105681463759e2 * t1003 * t11568 - t11474 + t11477 + t11480 - t11483 - t11486 - t11489 + t11493 + t11496 + t11500;
    (t11551, t11554, t11559, t11560, t11563, t11564, t11567, t11568, t11571)
}
