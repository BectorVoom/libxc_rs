//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1338/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1338(t22127: f64, t542: f64, t3255: f64, t7238: f64, t1409: f64, t7122: f64, t1319: f64, t3786: f64, t1419: f64, t7123: f64, t5498: f64, t1962: f64, t5526: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22128 = t542 * t22127;
    let t22131 = t3255 * t7238;
    let t22133 = t1409 * t7122;
    let t22134 = t22133 * t1319;
    let t22135 = t3786 * t22134;
    let t22138 = t7123 * t1419;
    let t22139 = t5498 * t22138;
    let t22142 = t1962 * t5526;
    (t22128, t22131, t22134, t22135, t22138, t22139, t22142)
}
