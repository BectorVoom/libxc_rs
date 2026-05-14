//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1369/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1369<F: Float>(t218: F, t219: F, t27287: F, t334: F, t3046: F, t836: F, t7996: F, t7999: F, t22296: F, t27358: F, t27361: F, t27363: F, t27367: F, t27370: F, t27373: F, t27377: F, t27381: F) -> (F, F, F, F) {
    let t27385 = t218 * t219 * t334 * t27287;
    let t27387 = t836 * t3046;
    let t27388 = t7996 * t27387;
    let t27390 = t7999 * t27387;
    let t27392 = 0.11038e1 * t22296 + 0.27595e0 * t27358 - 0.66228e0 * t27361 - 0.258925e1 * t27363 + 0.49671e0 * t27367 - 0.33114e0 * t27370 - 0.33114e0 * t27373 + 0.248355e0 * t27377 + 0.49671e0 * t27381 + 0.248355e0 * t27385 + 0.776775e1 * t27388 - 0.16504875e0 * t27390;
    (t27385, t27388, t27390, t27392)
}
