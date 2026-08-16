//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 603/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk603(t15361: f64, t2067: f64, t14237: f64, t14236: f64, t11674: f64, t14243: f64, t2841: f64, t14249: f64, t11599: f64, t2078: f64, t3369: f64, t3148: f64, t8450: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15362 = t2067 * t15361;
    let t15363 = t14237 * t15362;
    let t15364 = t14236 * t15363;
    let t15366 = t2067 * t11674;
    let t15367 = t14243 * t15366;
    let t15368 = t14236 * t15367;
    let t15370 = t2067 * t2841;
    let t15371 = t14249 * t15370;
    let t15372 = t14236 * t15371;
    let t15375 = t2078 * t11599;
    let t15376 = t3369 * t15375;
    let t15377 = t14236 * t15376;
    let t15379 = t8450 * t3148;
    (t15363, t15364, t15367, t15368, t15371, t15372, t15376, t15377, t15379)
}
