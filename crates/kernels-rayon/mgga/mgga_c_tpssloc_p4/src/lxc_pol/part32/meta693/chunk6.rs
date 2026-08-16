//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2152/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2152(t26395: f64, t5187: f64, t6637: f64, t6888: f64, t22892: f64, t22893: f64, t28148: f64, t1336: f64, t19732: f64, t19815: f64, t28178: f64, t3777: f64, t6987: f64, t6988: f64, t81080: f64, t90957: f64, t90962: f64, t90964: f64, t97036: f64, t97040: f64, t97043: f64, t97046: f64, t97049: f64, t97055: f64, t97059: f64, t97063: f64) -> f64 {
    let t97067 = t6888 * t6637 * t26395 * t5187;
    let t97070 = t22892 * t22893 * t28148;
    let t97075 = -0.52089578783527170488e-1_f64 * t81080 - 0.16449340668482264365e-1_f64 * t97036 - 0.16449340668482264365e-1_f64 * t97040 - 0.16449340668482264365e-1_f64 * t97043 + 0.49348022005446793095e-1_f64 * t97046 - 0.82246703342411321825e-2_f64 * t97049 - t1336 * t6987 * t19732 + 0.82246703342411321825e-2_f64 * t97055 - 0.49348022005446793095e-1_f64 * t97059 - 0.3289868133696452873e-1_f64 * t97063 - 0.3289868133696452873e-1_f64 * t97067 + 0.16449340668482264365e-1_f64 * t97070 - 2.0_f64 * t3777 * t28178 - t19815 * t6988 + t90957 - t90962 - t90964;
    t97075
}
