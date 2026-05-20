//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1668/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1668<F: Float>(t12627: F, t1269: F, t3566: F, t3727: F, t12640: F, t44842: F, t487: F, t1204: F, t1210: F, t1211: F, t1214: F, t12603: F, t12630: F, t12651: F, t12654: F, t12658: F, t12673: F, t12690: F, t12696: F, t1271: F, t13170: F, t13174: F, t13182: F, t13183: F, t225: F, t3552: F, t3556: F, t3569: F, t3572: F, t3585: F, t3729: F, t3732: F, t3739: F, t3791: F, t44321: F, t44845: F, t45406: F, t460: F, t494: F, t495: F) -> F {
    let t45427 = t12627 * t1269;
    let t45430 = t3566 * t3727;
    let t45433 = t12640 * t1269;
    let t45438 = t44842 * t487;
    let t45448 = F::cast_from(0.26341796731742046395e1_f64) * t1204 * t13170 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t45406 * t225 * t494 + F::cast_from(0.65854491829355115987e0_f64) * t44321 * t495 + F::cast_from(0.39512695097613069592e1_f64) * t3552 * t3729 + F::cast_from(0.79025390195226139183e1_f64) * t3572 * t12651 + F::cast_from(0.15805078039045227836e2_f64) * t12603 * t3739 - F::cast_from(0.15805078039045227836e2_f64) * t3556 * t13174 - F::cast_from(0.39512695097613069592e1_f64) * t12673 * t3791 - F::cast_from(0.39512695097613069592e1_f64) * t12654 * t3791 - F::cast_from(0.39512695097613069592e1_f64) * t12658 * t3585 - F::cast_from(0.15805078039045227836e2_f64) * t45427 * t12630 + F::cast_from(0.79025390195226139183e1_f64) * t45430 * t3569 + F::cast_from(0.15805078039045227836e2_f64) * t45433 * t3569 + F::cast_from(0.15805078039045227836e2_f64) * t3732 * t12696 + F::cast_from(0.15805078039045227836e2_f64) * t45438 * t1211 * t44845 + F::cast_from(0.26341796731742046395e1_f64) * t12690 * t1271 + F::cast_from(0.15805078039045227836e2_f64) * t1210 * t13182 * t13183 * t1214;
    t45448
}
