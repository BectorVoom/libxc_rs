//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 413/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk413<F: Float>(t2482: F, t27: F, t823: F, t136: F, t826: F, t221: F, t837: F, t737: F, t744: F, t185: F, t760: F, t128: F, t131: F, t2457: F, t2470: F, t684: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2484 = t2482 * t823 * t27;
    let t2485 = t826 * t136;
    let t2487 = t2485 * t221 * t837;
    let t2488 = t2484 * t2487;
    let t2490 = t737 * t737;
    let t2491 = 1.0 / t2490;
    let t2492 = t744 * t744;
    let t2494 = t185 * t185;
    let t2495 = 1.0 / t2494;
    let t2496 = t2491 * t2492 * t2495;
    let t2498 = 0.17315859105681463759e2 * t760 * t2496;
    let t2501 = 1.0 / t131 / t128 * t136;
    let t2502 = t2501 * t2457;
    let t2504 = t684 * t2470;
    (t2484, t2485, t2487, t2488, t2491, t2492, t2495, t2496, t2498, t2502, t2504)
}
