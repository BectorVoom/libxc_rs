//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 890/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk890(t326: f64, t6523: f64, t6458: f64, t2370: f64, t5728: f64, t6461: f64, t758: f64, t2099: f64, t2389: f64, t918: f64, t2380: f64, t3206: f64, t385: f64, t6443: f64, t6449: f64, t6453: f64, t6459: f64, t6464: f64, t6468: f64, t6472: f64, t6477: f64, t6480: f64, t6485: f64, t6489: f64, t6492: f64, t6509: f64, t6516: f64, t6520: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6524 = t6523 * t326;
    let t6525 = t6524 * t6458;
    let t6526 = t5728 * t2370;
    let t6527 = t6461 * t6526;
    let t6528 = t758 * t6527;
    let t6531 = t2099 * t2389;
    let t6532 = t918 * t6531;
    let t6534 = -t385 * t6443 / 96.0_f64 + t6449 / 144.0_f64 - t6453 / 96.0_f64 + 0.21437009059034868486e-3_f64 * t6459 * t6464 - 0.14291339372689912324e-3_f64 * t6468 - 0.64311027177104605458e-3_f64 * t3206 * t6472 - 0.17149607247227894789e-2_f64 * t6477 - 0.12862205435420921092e-2_f64 * t2380 * t6480 - 0.12862205435420921092e-2_f64 * t2380 * t6485 + 0.85748036236139473944e-3_f64 * t6489 - 0.42874018118069736972e-3_f64 * t6492 + 0.21437009059034868486e-3_f64 * t918 * t6509 + 0.12862205435420921092e-2_f64 * t6516 * t6520 - 0.12862205435420921092e-2_f64 * t6525 * t6528 + 0.42874018118069736972e-3_f64 * t6532;
    (t6524, t6525, t6526, t6527, t6528, t6531, t6532, t6534)
}
