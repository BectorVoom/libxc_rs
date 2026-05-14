//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1382/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1382<F: Float>(t3819: F, t6233: F, t881: F, t9973: F, t2317: F, t3801: F, t1209: F, t18513: F, t18740: F, t18878: F, t22740: F, t2297: F, t2312: F, t2313: F, t2318: F, t2321: F, t27394: F, t27481: F, t27484: F, t27493: F, t27496: F, t27498: F, t3136: F, t3139: F, t3806: F, t3823: F, t6282: F, t6323: F, t8098: F, t8102: F, t882: F, t890: F, t891: F, t9964: F, t9985: F, t9992: F) -> (F,) {
    let t27675 = t3819 * t6233;
    let t27694 = t9973 * t881;
    let t27699 = t3801 * t2317;
    let t27706 = -0.10389515463408878255e3 * t6323 * t9985 * t2297 + 0.17315859105681463759e2 * t2318 * t9985 * t2312 + 0.10254018858216406658e4 * t6282 * t27675 * t2297 + 0.34631718211362927518e2 * t2318 * t3139 * t8098 + 0.10254018858216406658e4 * t6282 * t9992 * t2312 + 0.91082604192152556044e5 * t18878 * t3806 * t18513 * t2297 - t27481 + t27484 + 0.5848223622634646207e0 * t882 * t27394 * t890 + 0.17315859105681463759e2 * t18740 * t3823 + 0.11696447245269292414e1 * t27694 * t891 + 0.5848223622634646207e0 * t9964 * t2313 + 0.17315859105681463759e2 * t27699 * t2321 + 0.11696447245269292414e1 * t22740 * t1209 + 0.23392894490538584828e1 * t8102 * t3136 - t27493 - t27496 - t27498;
    (t27706,)
}
