//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1149/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1149<F: Float>(t2868: F, t38149: F, t42234: F, t42248: F, t42250: F, t44444: F, t44445: F, t44446: F, t44450: F, t48027: F, t48029: F, t48031: F, t48036: F, t48039: F, t48041: F, t48043: F, t48047: F, t48049: F, t9332: F) -> F {
    let t49818 = -F::cast_from(0.638468998399467591e-4_f64) * t48027 + F::cast_from(0.638468998399467591e-4_f64) * t48029 + F::cast_from(0.212822999466489197e-4_f64) * t48031 - F::cast_from(0.212822999466489197e-4_f64) * t48036 - F::cast_from(0.85129199786595678799e-5_f64) * t48039 - F::cast_from(0.5107751987195740728e-4_f64) * t48041 + F::cast_from(0.5107751987195740728e-4_f64) * t48043 - F::cast_from(0.81823984962736025192e-1_f64) * t48047 - F::cast_from(0.20455996240684006298e-1_f64) * t48049 - F::cast_from(0.7684513755465791136e-2_f64) * t42234 + t44444 + t44445 + t44446 + F::cast_from(0.1440846329149835838e-2_f64) * t42248 - F::cast_from(0.1440846329149835838e-2_f64) * t42250 - t44450 - t38149 - F::cast_from(0.11974241701863808564e0_f64) * t2868 * t9332;
    t49818
}
