//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 864/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk864(t118: f64, t1986: f64, t615: f64, t665: f64, t7717: f64, t2046: f64, t2049: f64, t2323: f64, t15039: f64, t2160: f64, t638: f64, t14286: f64, t551: f64) -> (f64, f64, f64, f64) {
    let t75498 = t1986 * t118 * t665 * t615;
    let t75500 = 0.1064114997332445985e-4_f64 * t7717 * t75498;
    let t75508 = t2046 * t2049 * t2323;
    let t75513 = t638 * t2160 * t15039;
    let t75515 = t14286 * t551;
    (t75500, t75508, t75513, t75515)
}
