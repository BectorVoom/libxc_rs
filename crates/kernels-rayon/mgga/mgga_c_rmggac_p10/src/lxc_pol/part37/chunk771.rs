//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 771/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk771(t2046: f64, t2050: f64, t2408: f64, t31: f64, t2039: f64, t2406: f64, t270: f64, t638: f64, t34738: f64, t656: f64, t8982: f64, t36471: f64, t8985: f64) -> (f64, f64, f64, f64) {
    let t73953 = t2046 * t2050 * t2408 * t31;
    let t73957 = t638 * t2039 * t2406 * t270;
    let t73960 = t34738 * t656 * t8982;
    let t73963 = t36471 * t656 * t8985;
    (t73953, t73957, t73960, t73963)
}
