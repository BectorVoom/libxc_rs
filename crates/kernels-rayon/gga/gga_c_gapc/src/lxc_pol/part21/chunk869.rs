//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 869/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk869(t1096: f64, t2822: f64, t2470: f64, t1066: f64, t2468: f64, t2902: f64, t761: f64, t3221: f64, t1474: f64, t277: f64, t1051: f64, t2043: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10088 = t1096 * t2822;
    let t10091 = t1096 * t2470;
    let t10099 = t1066 * t2468;
    let t10102 = t2902 * t761;
    let t10103 = t10102 * t3221;
    let t10105 = t1474 * t277;
    let t10106 = t10105 * t3221;
    let t10108 = t2043 * t1051;
    (t10088, t10091, t10099, t10102, t10103, t10105, t10106, t10108)
}
