//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1155/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1155(t2240: f64, t24525: f64, t1860: f64, t2110: f64, t22493: f64, t22519: f64, t22527: f64, t22531: f64, t22534: f64, t22537: f64, t22546: f64, t22549: f64, t24505: f64, t24508: f64, t24511: f64, t24514: f64, t24517: f64, t24520: f64, t6486: f64, t6492: f64, t6495: f64, t7246: f64, t7256: f64, t7259: f64) -> (f64, f64) {
    let t24526 = t2240 * t24525;
    let t24541 = -t22493 * t2110 / 6.0_f64 - t6486 * t7256 / 3.0_f64 - t6486 * t7259 / 3.0_f64 - t1860 * t24505 / 6.0_f64 - t1860 * t24508 / 3.0_f64 - t1860 * t24511 / 6.0_f64 - 5.0_f64 * t24514 * t22546 - 10.0_f64 / 3.0_f64 * t22549 * t24517 + 5.0_f64 / 3.0_f64 * t24520 * t6492 + 2.0_f64 / 3.0_f64 * t22519 * t2110 + 5.0_f64 / 3.0_f64 * t24526 * t6492 + 5.0_f64 / 3.0_f64 * t7246 * t22527 + 5.0_f64 / 6.0_f64 * t7246 * t22531 + t22534 * t2110 / 3.0_f64 + t22537 * t2110 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t6495 * t7256 + 2.0_f64 / 3.0_f64 * t6495 * t7259;
    (t24526, t24541)
}
