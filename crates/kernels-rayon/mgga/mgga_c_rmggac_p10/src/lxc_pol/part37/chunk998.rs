//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 998/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk998(t76315: f64, t352: f64, t77960: f64, t8940: f64, t25877: f64, t77094: f64, t25854: f64, t77097: f64, t76323: f64, t25820: f64, t77085: f64, t27101: f64, t77088: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t78040 = 0.20455996240684006296e-1_f64 * t76315;
    let t78046 = 0.11974241701863808564e0_f64 * t8940 * t77960 * t352;
    let t78047 = t25877 * t77094;
    let t78048 = 0.17961362552795712846e0_f64 * t78047;
    let t78049 = t25854 * t77097;
    let t78050 = 0.8980681276397856423e-1_f64 * t78049;
    let t78051 = 0.14967802127329760705e-1_f64 * t76323;
    let t78052 = t25820 * t77085;
    let t78053 = 0.8980681276397856423e-1_f64 * t78052;
    let t78054 = t27101 * t77088;
    (t78040, t78046, t78048, t78050, t78051, t78053, t78054)
}
