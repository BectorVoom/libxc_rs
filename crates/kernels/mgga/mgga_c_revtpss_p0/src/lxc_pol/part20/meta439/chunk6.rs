//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1669/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1669<F: Float>(t45384: F, t487: F, t1269: F, t3552: F, t44420: F, t12690: F, t1210: F, t1211: F, t1214: F, t12600: F, t12603: F, t12622: F, t12630: F, t12633: F, t12651: F, t12654: F, t12673: F, t12695: F, t1276: F, t1277: F, t1295: F, t13174: F, t13177: F, t13184: F, t17973: F, t17986: F, t3556: F, t3567: F, t3568: F, t3569: F, t3572: F, t3575: F, t3576: F, t3584: F, t3732: F, t3736: F, t3739: F, t3790: F, t3791: F, t44878: F) -> F {
    let t45449 = t45384 * t487;
    let t45464 = t3552 * t1269;
    let t45482 = t44420 * t487;
    let t45487 = t12690 * t487;
    let t45494 = -F::cast_from(0.15805078039045227836e2_f64) * t45449 * t12630 - F::cast_from(0.79025390195226139183e1_f64) * t3567 * t1277 * t3568 * t3790 - F::cast_from(0.15805078039045227836e2_f64) * t17986 * t3736 * t1214 * t12695 - F::cast_from(0.15805078039045227836e2_f64) * t17973 * t1276 * t3584 * t3575 - F::cast_from(0.79025390195226139183e1_f64) * t45464 * t1295 - F::cast_from(0.65854491829355115987e0_f64) * t1210 * t1211 * t44878 + F::cast_from(0.79025390195226139183e1_f64) * t12673 * t3739 + F::cast_from(0.15805078039045227836e2_f64) * t13177 * t3576 + F::cast_from(0.79025390195226139183e1_f64) * t3556 * t12651 - F::cast_from(0.15805078039045227836e2_f64) * t3732 * t13184 + F::cast_from(0.79025390195226139183e1_f64) * t12654 * t3739 - F::cast_from(0.79025390195226139183e1_f64) * t12603 * t3791 + F::cast_from(0.79025390195226139183e1_f64) * t45482 * t3569 - F::cast_from(0.15805078039045227836e2_f64) * t12633 * t12600 - F::cast_from(0.26341796731742046395e1_f64) * t45487 * t1295 - F::cast_from(0.26341796731742046395e1_f64) * t3572 * t12622 - F::cast_from(0.15805078039045227836e2_f64) * t3572 * t13174;
    t45494
}
