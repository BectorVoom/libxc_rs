//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 893/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk893(t1469: f64, t27: f64, t16129: f64, t1966: f64, t221: f64, t69002: f64, t15379: f64, t68499: f64, t326: f64, t40998: f64, t1986: f64, t74179: f64) -> (f64, f64, f64) {
    let t75976 = t27 * t1469;
    let t75978 = t1966 * t69002 * t221 * t16129 * t75976;
    let t75993 = t15379 * t68499;
    let t75995 = t326 * t40998;
    let t75997 = t74179 * t1986 * t75995;
    (t75978, t75993, t75997)
}
