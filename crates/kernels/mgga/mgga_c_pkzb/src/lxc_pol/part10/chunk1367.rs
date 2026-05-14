//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1367/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1367<F: Float>(t3052: F, t7958: F, t2215: F, t836: F, t9798: F, t2209: F, t9811: F, t18480: F, t2204: F, t3743: F, t3747: F, t6149: F, t18492: F, t6165: F, t22265: F, t22284: F, t22287: F, t22290: F, t22293: F) -> (F, F, F, F, F, F, F, F) {
    let t27331 = t3052 * t7958;
    let t27334 = t2215 * t9798 * t836;
    let t27336 = t9811 * t2209;
    let t27339 = t18480 * t3743 * t2204;
    let t27342 = t6149 * t3747 * t2204;
    let t27345 = t18492 * t3743 * t2204;
    let t27348 = t6165 * t3747 * t2204;
    let t27355 = 0.16504875e0 * t27331 + 0.16504875e0 * t27334 + 0.82524375e-1 * t27336 - 0.485484375e1 * t27339 + 0.19419375e1 * t27342 + 0.6189328125e-1 * t27345 - 0.412621875e-1 * t27348 - 0.33114e0 * t22265 - 0.33114e0 * t22284 - 0.66228e0 * t22287 - 0.14717333333333333333e1 * t22290 + 0.11038e1 * t22293;
    (t27331, t27334, t27336, t27339, t27342, t27345, t27348, t27355)
}
