//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1029/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1029<F: Float>(t3311: F, t459: F, t2507: F, t995: F, t2528: F, t987: F, t3337: F, t3314: F, t4794: F, t440: F, t8: F, t973: F, t1429: F, t1424: F, t3318: F, t4803: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8604 = t3311 * t459;
    let t8607 = t2507 * t995;
    let t8610 = t987 * t2528;
    let t8615 = t3337 * t459;
    let t8620 = t4794 * t3314;
    let t8621 = t8620 * t440;
    let t8624 = t973 * t8;
    let t8625 = t8624 * t1429;
    let t8630 = t1424 * t3318;
    let t8631 = t8630 * t440;
    let t8635 = -t1429 - 3.0 * t4803;
    (t8604, t8607, t8610, t8615, t8620, t8621, t8624, t8625, t8630, t8631, t8635)
}
