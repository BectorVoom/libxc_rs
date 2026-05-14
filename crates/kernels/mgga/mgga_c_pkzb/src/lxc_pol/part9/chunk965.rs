//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 965/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk965<F: Float>(t2380: F, t8470: F, t3026: F, t919: F, t921: F, t2381: F, t3185: F, t3206: F, t3235: F, t8408: F, t8411: F, t8415: F, t8420: F, t8424: F, t8428: F, t8432: F, t8435: F, t8438: F, t8442: F, t8447: F, t8450: F, t8453: F, t8458: F, t8460: F, t8464: F, t8469: F) -> (F, F) {
    let t8472 = 0.57165357490759649296e-3 * t2380 * t8470;
    let t8473 = t3026 * t919;
    let t8474 = t8473 * t921;
    let t8475 = t2381 * t8474;
    let t8478 = t8408 + 0.25724410870841842184e-2 * t3235 * t8411 + 0.12862205435420921092e-2 * t3235 * t8415 - 0.51448821741683684368e-2 * t3235 * t8420 + 0.42874018118069736972e-3 * t3185 * t8424 + 0.12862205435420921092e-2 * t8428 * t8432 - 0.12862205435420921092e-2 * t8435 * t8438 - 0.42874018118069736972e-3 * t3206 * t8442 - 0.21437009059034868486e-3 * t3206 * t8447 + 0.21437009059034868486e-3 * t8450 * t8453 - t8458 - 0.85748036236139473944e-3 * t2380 * t8460 - 0.42874018118069736972e-3 * t2380 * t8464 + t8469 - t8472 - 0.85748036236139473944e-3 * t2380 * t8475;
    (t8474, t8478)
}
