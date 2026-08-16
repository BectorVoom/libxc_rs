//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 769/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk769(t107: f64, t31730: f64, t544: f64, t6540: f64, t986: f64, t2299: f64, t2754: f64, t3394: f64, t4130: f64, t10417: f64, t1397: f64, t10241: f64, t9448: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34558 = t544 * t31730 * t107;
    let t34600 = t6540 * t986;
    let t34604 = t2299 * t2754;
    let t34688 = t4130 * t3394;
    let t34777 = t1397 * t10417;
    let t34814 = t9448 * t10241;
    (t34558, t34600, t34604, t34688, t34777, t34814)
}
