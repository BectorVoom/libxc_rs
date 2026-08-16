//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 979/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk979(t169: f64, t301: f64, t4867: f64, t784: f64, t159: f64, t18075: f64, t285: f64, t2331: f64, t274: f64, t1473: f64, t1488: f64, t1492: f64) -> (f64, f64, f64, f64, f64) {
    let t18116 = t169 * t784 * t4867 * t301;
    let t18122 = 0.3831185177913978998e-1_f64 * t18075 * t159 * t285;
    let t18126 = 0.52404510650723236824e1_f64 * t169 * t2331 * t274 * t301;
    let t18129 = t1473 * t1488;
    let t18131 = t1473 * t1492;
    (t18116, t18122, t18126, t18129, t18131)
}
