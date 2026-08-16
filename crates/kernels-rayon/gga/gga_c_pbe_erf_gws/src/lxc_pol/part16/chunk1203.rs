//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1203/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1203(t1176: f64, t2332: f64, t903: f64, t3993: f64, t13788: f64, t13972: f64, t13865: f64, t51666: f64, t14001: f64, t2412: f64, t1180: f64, t6589: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51818 = t1176 * t2332 * t903;
    let t51819 = t51818 * t3993;
    let t51827 = t13972 * t13788;
    let t51829 = t51666 * t13865;
    let t51864 = t14001 * t2412;
    let t51869 = t1176 * t6589 * t1180;
    (t51818, t51819, t51827, t51829, t51864, t51869)
}
