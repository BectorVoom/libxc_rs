//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 947/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk947(t305: f64, t38812: f64, t128: f64, t30526: f64, t8645: f64, t338: f64, t6444: f64, t8649: f64, t39665: f64, t5259: f64, t2392: f64, t839: f64) -> (f64, f64, f64, f64, f64) {
    let t40814 = t305 * t38812;
    let t40823 = t30526 * t128;
    let t40824 = t40823 * t8645;
    let t40826 = t6444 * t338;
    let t40827 = t40826 * t8649;
    let t40831 = t5259 * t39665;
    let t40833 = t2392 * t839;
    (t40814, t40824, t40827, t40831, t40833)
}
