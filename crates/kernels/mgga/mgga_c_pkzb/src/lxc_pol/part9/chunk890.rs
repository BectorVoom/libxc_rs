//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 890/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk890<F: Float>(t326: F, t6523: F, t6458: F, t2370: F, t5728: F, t6461: F, t758: F, t2099: F, t2389: F, t918: F, t2380: F, t3206: F, t385: F, t6443: F, t6449: F, t6453: F, t6459: F, t6464: F, t6468: F, t6472: F, t6477: F, t6480: F, t6485: F, t6489: F, t6492: F, t6509: F, t6516: F, t6520: F) -> (F, F, F, F, F, F, F, F) {
    let t6524 = t6523 * t326;
    let t6525 = t6524 * t6458;
    let t6526 = t5728 * t2370;
    let t6527 = t6461 * t6526;
    let t6528 = t758 * t6527;
    let t6531 = t2099 * t2389;
    let t6532 = t918 * t6531;
    let t6534 = -t385 * t6443 / F::new(96.0) + t6449 / F::new(144.0) - t6453 / F::new(96.0) + F::cast_from(0.21437009059034868486e-3_f64) * t6459 * t6464 - F::cast_from(0.14291339372689912324e-3_f64) * t6468 - F::cast_from(0.64311027177104605458e-3_f64) * t3206 * t6472 - F::cast_from(0.17149607247227894789e-2_f64) * t6477 - F::cast_from(0.12862205435420921092e-2_f64) * t2380 * t6480 - F::cast_from(0.12862205435420921092e-2_f64) * t2380 * t6485 + F::cast_from(0.85748036236139473944e-3_f64) * t6489 - F::cast_from(0.42874018118069736972e-3_f64) * t6492 + F::cast_from(0.21437009059034868486e-3_f64) * t918 * t6509 + F::cast_from(0.12862205435420921092e-2_f64) * t6516 * t6520 - F::cast_from(0.12862205435420921092e-2_f64) * t6525 * t6528 + F::cast_from(0.42874018118069736972e-3_f64) * t6532;
    (t6524, t6525, t6526, t6527, t6528, t6531, t6532, t6534)
}
