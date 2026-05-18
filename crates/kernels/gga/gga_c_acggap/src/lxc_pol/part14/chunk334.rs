//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 334/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk334<F: Float>(t1180: F, t127: F, t1441: F, t1446: F, t1451: F, t1456: F, t1462: F, t1466: F, t1470: F, t1490: F, t1494: F, t1498: F, t1503: F, t1507: F, t1511: F, t1514: F, t1516: F, t1526: F, t1531: F, t1535: F, t335: F, t367: F, t418: F) -> F {
    let t1538 = F::new(0.17149607247227894789e-2) * t418 * t1441 - F::new(0.17149607247227894789e-2) * t418 * t1446 - F::new(0.85748036236139473944e-3) * t418 * t1451 - F::new(0.85748036236139473944e-3) * t418 * t1456 + F::new(0.12862205435420921092e-2) * t418 * t1462 - F::new(0.42874018118069736972e-3) * t1180 * t1466 + F::new(0.42874018118069736972e-3) * t1180 * t1470 + t127 * t1490 / F::new(96.0) - t335 * t1494 / F::new(48.0) - t335 * t1498 / F::new(48.0) - t335 * t1503 / F::new(48.0) - t367 * t1507 / F::new(96.0) - t367 * t1511 / F::new(96.0) + F::new(7.0) / F::new(288.0) * t1514 + F::new(7.0) / F::new(144.0) * t1516 - t367 * t1526 / F::new(96.0) + F::new(0.42874018118069736972e-3) * t1531 * t1535;
    t1538
}
