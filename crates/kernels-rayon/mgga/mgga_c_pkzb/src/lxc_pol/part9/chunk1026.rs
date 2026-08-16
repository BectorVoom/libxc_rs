//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1026/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1026(t2380: f64, t8392: f64, t1167: f64, t179: f64, t6380: f64, t404: f64, t1238: f64, t2414: f64, t2418: f64, t3185: f64, t6434: f64, t6449: f64, t6453: f64, t6468: f64, t6477: f64, t6489: f64, t6492: f64, t6532: f64, t8377: f64, t8382: f64, t8386: f64, t8389: f64) -> f64 {
    let t8394 = 0.57165357490759649296e-3_f64 * t2380 * t8392;
    let t8397 = t179 * t6380 * t1167;
    let t8398 = t404 * t8397;
    let t8405 = t6434 / 144.0_f64 + t6449 / 216.0_f64 - t6453 / 288.0_f64 + 0.12862205435420921092e-2_f64 * t2380 * t8377 + 0.85748036236139473944e-3_f64 * t3185 * t8382 - t8386 - 0.95275595817932748826e-4_f64 * t6468 + t8389 + 0.22866142996303859718e-2_f64 * t1238 * t2418 - t8394 - 0.57165357490759649296e-3_f64 * t6477 + 0.95275595817932748827e-4_f64 * t8398 - 0.68598428988911579154e-2_f64 * t1238 * t2414 + 0.28582678745379824648e-3_f64 * t6489 - 0.14291339372689912324e-3_f64 * t6492 + 0.14291339372689912324e-3_f64 * t6532;
    t8405
}
