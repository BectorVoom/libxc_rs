//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1249/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1249<F: Float>(t12: F, t1499: F, t8770: F, t16584: F, t496: F, t8777: F, t1429: F, t1541: F, t16232: F, t1642: F, t1643: F, t1646: F, t19653: F, t23948: F, t2540: F, t3363: F, t3366: F, t439: F, t4803: F, t5093: F, t6767: F, t78: F, t8721: F, t8726: F, t8729: F, zeta_threshold: F) -> (F, F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t24542 = t8770 * t1499;
    let t24543 = 0.5848223622634646207e0 * t24542;
    let t24544 = 24.0 * t16584;
    let t24545 = t496 * t8777;
    let t24546 = 8.0 * t24545;
    let t24570 = piecewise3(t84, 0.0, 40.0 / 81.0 * t16232 * t3363 * t1643 - 64.0 / 27.0 * t6767 * t23948 - 8.0 / 27.0 * t8721 * t1646 + 32.0 / 9.0 * t1642 * t78 * t1541 + 16.0 / 9.0 * t2540 * t1429 - 16.0 / 3.0 * t2540 * t4803 - 8.0 / 27.0 * t5093 * t3366 * t1643 + 8.0 / 9.0 * t1642 * t8729 * t439 + 4.0 / 9.0 * t8726 * t1646 + t19653);
    (t24543, t24544, t24546, t24570)
}
