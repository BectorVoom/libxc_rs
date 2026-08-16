//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1004/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1004(t1971: f64, t236: f64, t5567: f64, t8517: f64, t2160: f64, t638: f64, t8850: f64, t8854: f64, t131: f64, t4999: f64, t639: f64, t71: f64) -> (f64, f64, f64, f64) {
    let t42011 = t8517 * t1971 * t236 * t5567;
    let t42023 = t638 * t2160 * t8850;
    let t42026 = t638 * t2160 * t8854;
    let t42032 = t638 * t639 * t71 * t4999 * t131;
    (t42011, t42023, t42026, t42032)
}
