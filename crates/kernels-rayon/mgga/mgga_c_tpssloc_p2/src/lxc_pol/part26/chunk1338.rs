//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1338/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1338(t2109: f64, t83718: f64, t22550: f64, t7255: f64, t83728: f64, t83737: f64, t22534: f64, t22549: f64, t24508: f64, t24511: f64, t24514: f64, t24517: f64, t6486: f64, t7256: f64, t7259: f64, t83717: f64, t83722: f64, t83734: f64, t83778: f64) -> f64 {
    let t85463 = t2109 * t83718;
    let t85470 = t7255 * t22550;
    let t85473 = t2109 * t83728;
    let t85476 = t2109 * t83737;
    let t85479 = -t6486 * t24508 - t6486 * t24511 / 2.0_f64 + t22534 * t7256 + t22534 * t7259 - 15.0_f64 * t24514 * t83734 + 30.0_f64 * t83717 * t85463 - 10.0_f64 * t83722 * t24517 - 5.0_f64 * t83778 * t24517 - 10.0_f64 * t22549 * t85470 - 10.0_f64 * t22549 * t85473 - 5.0_f64 * t22549 * t85476;
    t85479
}
