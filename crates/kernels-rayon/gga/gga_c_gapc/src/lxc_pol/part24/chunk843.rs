//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 843/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk843(t10078: f64, t7591: f64, t941: f64, t10075: f64, t2902: f64, t761: f64, t3221: f64, t1474: f64, t277: f64, t1051: f64, t2043: f64, t6808: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10079 = t7591 * t941 * t10078;
    let t10080 = t10075 * t10079;
    let t10102 = t2902 * t761;
    let t10103 = t10102 * t3221;
    let t10105 = t1474 * t277;
    let t10106 = t10105 * t3221;
    let t10108 = t2043 * t1051;
    let t10110 = t2902 * t6808;
    (t10079, t10080, t10102, t10103, t10105, t10106, t10108, t10110)
}
