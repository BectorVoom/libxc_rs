//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 776/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk776(t3140: f64, t9221: f64, t13868: f64, t14230: f64, t1525: f64, t2067: f64, t26: f64, t3369: f64, t15227: f64, t68444: f64, t68386: f64, t7248: f64, t9122: f64) -> (f64, f64, f64, f64) {
    let t74035 = t9221 * t3140;
    let t74036 = t74035 * t13868;
    let t74041 = t14230 * t3369 * t2067 * t26 * t1525;
    let t74043 = t68444 * t15227;
    let t74046 = t68386 * t7248 * t9122;
    (t74036, t74041, t74043, t74046)
}
