//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 588/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk588(t13940: f64, t15109: f64, t2367: f64, t36: f64, t2079: f64, t262: f64, t14290: f64, t556: f64, t14293: f64, t2842: f64, t27: f64, t29: f64, t570: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15122 = t13940 * t15109;
    let t15128 = t36 * t2367;
    let t15130 = t2079 * t262 * t15128;
    let t15132 = t14290 * t556;
    let t15134 = t14293 * t2842;
    let t15137 = t27 * t29 * t570;
    (t15122, t15128, t15130, t15132, t15134, t15137)
}
