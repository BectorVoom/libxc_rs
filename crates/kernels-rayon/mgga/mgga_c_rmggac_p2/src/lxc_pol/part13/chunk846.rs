//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 846/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk846(t1540: f64, t880: f64, t2141: f64, t2392: f64, t798: f64, t26287: f64, t4048: f64, t30204: f64, t4905: f64, t26291: f64, t16156: f64, t9096: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38973 = t1540 * t880;
    let t38974 = t38973 * t2141;
    let t38977 = t2392 * t798;
    let t38978 = t26287 * t38977;
    let t38980 = t2392 * t4048;
    let t38981 = t30204 * t38980;
    let t38983 = t2392 * t4905;
    let t38984 = t26291 * t38983;
    let t38986 = t16156 * t9096;
    (t38974, t38977, t38978, t38980, t38981, t38983, t38984, t38986)
}
