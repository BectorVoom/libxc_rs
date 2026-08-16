//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1333/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1333(t21078: f64, t5425: f64, t531: f64, t6957: f64, t11332: f64, t833: f64, t544: f64, t6964: f64, t1319: f64, t5457: f64, t518: f64, t1419: f64) -> (f64, f64, f64, f64, f64) {
    let t22044 = t5425 * t21078;
    let t22047 = t6957 * t531;
    let t22049 = t11332 * t22047 * t833;
    let t22053 = t544 * t6964;
    let t22054 = t22053 * t1319;
    let t22055 = t5457 * t22054;
    let t22058 = t518 * t6964;
    let t22059 = t22058 * t1419;
    (t22044, t22049, t22054, t22055, t22059)
}
