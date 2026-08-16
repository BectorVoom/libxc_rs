//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1149/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1149(t2868: f64, t38149: f64, t42234: f64, t42248: f64, t42250: f64, t44444: f64, t44445: f64, t44446: f64, t44450: f64, t48027: f64, t48029: f64, t48031: f64, t48036: f64, t48039: f64, t48041: f64, t48043: f64, t48047: f64, t48049: f64, t9332: f64) -> f64 {
    let t49818 = -0.638468998399467591e-4_f64 * t48027 + 0.638468998399467591e-4_f64 * t48029 + 0.212822999466489197e-4_f64 * t48031 - 0.212822999466489197e-4_f64 * t48036 - 0.85129199786595678799e-5_f64 * t48039 - 0.5107751987195740728e-4_f64 * t48041 + 0.5107751987195740728e-4_f64 * t48043 - 0.81823984962736025192e-1_f64 * t48047 - 0.20455996240684006298e-1_f64 * t48049 - 0.7684513755465791136e-2_f64 * t42234 + t44444 + t44445 + t44446 + 0.1440846329149835838e-2_f64 * t42248 - 0.1440846329149835838e-2_f64 * t42250 - t44450 - t38149 - 0.11974241701863808564e0_f64 * t2868 * t9332;
    t49818
}
