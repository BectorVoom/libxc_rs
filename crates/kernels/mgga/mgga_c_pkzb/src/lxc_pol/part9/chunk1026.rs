//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1026/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1026<F: Float>(t2380: F, t8392: F, t1167: F, t179: F, t6380: F, t404: F, t1238: F, t2414: F, t2418: F, t3185: F, t6434: F, t6449: F, t6453: F, t6468: F, t6477: F, t6489: F, t6492: F, t6532: F, t8377: F, t8382: F, t8386: F, t8389: F) -> F {
    let t8394 = F::new(0.57165357490759649296e-3) * t2380 * t8392;
    let t8397 = t179 * t6380 * t1167;
    let t8398 = t404 * t8397;
    let t8405 = t6434 / F::new(144.0) + t6449 / F::new(216.0) - t6453 / F::new(288.0) + F::new(0.12862205435420921092e-2) * t2380 * t8377 + F::new(0.85748036236139473944e-3) * t3185 * t8382 - t8386 - F::new(0.95275595817932748826e-4) * t6468 + t8389 + F::new(0.22866142996303859718e-2) * t1238 * t2418 - t8394 - F::new(0.57165357490759649296e-3) * t6477 + F::new(0.95275595817932748827e-4) * t8398 - F::new(0.68598428988911579154e-2) * t1238 * t2414 + F::new(0.28582678745379824648e-3) * t6489 - F::new(0.14291339372689912324e-3) * t6492 + F::new(0.14291339372689912324e-3) * t6532;
    t8405
}
