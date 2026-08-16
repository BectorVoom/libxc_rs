//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 778/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk778(t36204: f64, t7778: f64, t866: f64, t305: f64, t2067: f64, t25525: f64, t2078: f64, t3839: f64, t262: f64, t35917: f64, t7785: f64, t35844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36205 = 0.51855529564861513904e-1_f64 * t36204;
    let t36247 = t7778 * t866;
    let t36248 = t305 * t36247;
    let t36250 = t25525 * t2067;
    let t36254 = t3839 * t2078;
    let t36268 = t262 * t35917;
    let t36269 = t7785 * t36268;
    let t36271 = t262 * t35844;
    (t36205, t36247, t36248, t36250, t36254, t36268, t36269, t36271)
}
