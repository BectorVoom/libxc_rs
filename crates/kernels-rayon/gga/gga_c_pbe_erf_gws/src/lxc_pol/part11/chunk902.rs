//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 902/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk902(t155: f64, t174: f64, t4508: f64, t4511: f64, t1268: f64, t4537: f64, t1216: f64, t4605: f64, t4623: f64, t470: f64, t1322: f64, t4619: f64) -> (f64, f64, f64, f64) {
    let t18432 = 0.68733717152873822009e1_f64 * t174 * t155 * t4508 * t4511;
    let t18435 = 0.71233333333333333333e-1_f64 * t174 * t1268 * t4537;
    let t18439 = 0.62336721237753107879e3_f64 * t470 * t4605 * t1216 * t4623;
    let t18442 = t1322 * t4619;
    (t18432, t18435, t18439, t18442)
}
