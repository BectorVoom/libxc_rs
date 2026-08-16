//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 500/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk500(t14051: f64, t3119: f64, t14011: f64, t1996: f64, t13862: f64, t2003: f64, t3120: f64, t323: f64, t1008: f64, t140: f64, t212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14052 = t14051 * t3119;
    let t14053 = t14011 * t1996;
    let t14054 = t14052 * t14053;
    let t14056 = t13862 * t2003;
    let t14057 = t3120 * t14056;
    let t14059 = t14011 * t323;
    let t14060 = t3120 * t14059;
    let t14063 = t212 * t1008 * t140;
    (t14052, t14053, t14054, t14056, t14057, t14059, t14060, t14063)
}
