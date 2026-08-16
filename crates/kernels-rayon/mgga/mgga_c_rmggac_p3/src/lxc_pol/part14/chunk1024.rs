//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1024/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1024(t41377: f64, t2103: f64, t41036: f64, t2118: f64, t36175: f64, t36184: f64, t36192: f64, t36194: f64, t36201: f64, t36205: f64, t41363: f64, t41365: f64, t41367: f64, t41368: f64, t41371: f64, t41373: f64, t41375: f64) -> f64 {
    let t41378 = 0.18183107769496894486e-1_f64 * t41377;
    let t41379 = t2103 * t41036;
    let t41380 = 0.24244143692662525982e-1_f64 * t41379;
    let t41381 = t2118 * t41036;
    let t41383 = -0.10620923284048465071e-2_f64 * t36175 + 0.3540307761349488357e-2_f64 * t36184 + 0.33335697577410973225e-1_f64 * t41363 + 0.88704377798256624947e-3_f64 * t41365 - t41367 + t41368 + 0.74346462988339255497e-2_f64 * t36192 + 0.88507694033737208925e-3_f64 * t36194 + t36201 + 0.53218852008283593618e-1_f64 * t41371 + 0.53218852008283593618e-1_f64 * t41373 - t36205 - 0.10584045078201074568e-3_f64 * t41375 - t41378 + t41380 + 0.56448240417072397696e-3_f64 * t41381;
    t41383
}
