//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1939/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1939(t27363: f64, t67: f64, t1864: f64, t1860: f64, t2110: f64, t24520: f64, t24526: f64, t26055: f64, t26063: f64, t26067: f64, t26090: f64, t27332: f64, t27341: f64, t6486: f64, t6492: f64, t6495: f64, t7246: f64, t7256: f64, t7259: f64, t7432: f64, t7435: f64, t7975: f64, t7978: f64) -> (f64, f64, f64) {
    let t27364 = t27363 * t67;
    let t27365 = t27364 * t1864;
    let t27368 = t7435 * t7256 / 3.0_f64 + t7435 * t7259 / 3.0_f64 + 5.0_f64 / 6.0_f64 * t27332 * t6492 + t6495 * t7975 / 3.0_f64 + 5.0_f64 / 6.0_f64 * t7246 * t26090 + t6495 * t7978 / 3.0_f64 + 5.0_f64 / 6.0_f64 * t27341 * t6492 + t26055 * t2110 / 3.0_f64 + 5.0_f64 / 6.0_f64 * t24520 * t7432 + 5.0_f64 / 6.0_f64 * t24526 * t7432 + 5.0_f64 / 6.0_f64 * t7246 * t26063 + 5.0_f64 / 6.0_f64 * t7246 * t26067 - t6486 * t7975 / 6.0_f64 - t1860 * t27365 / 6.0_f64;
    (t27364, t27365, t27368)
}
