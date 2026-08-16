//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1037/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1037(t153: f64, t155: f64, t4080: f64, t121: f64, t4524: f64, t169: f64, t4529: f64, t1406: f64, t4780: f64, t1535: f64, t15478: f64, t4324: f64, t9448: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18096 = t153 / t4080 / t155;
    let t18310 = t121 * t4524;
    let t18313 = t169 * t4529;
    let t18337 = t1406 * t4780;
    let t18362 = t1535 * t15478;
    let t18364 = t9448 * t4324;
    (t18096, t18310, t18313, t18337, t18362, t18364)
}
