//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1210/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1210(t13865: f64, t51666: f64, t14001: f64, t2412: f64, t1176: f64, t1180: f64, t6589: f64, t13987: f64, t894: f64, t13855: f64, t13953: f64, t1193: f64, t2182: f64, t353: f64, t8599: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51829 = t51666 * t13865;
    let t51864 = t14001 * t2412;
    let t51869 = t1176 * t6589 * t1180;
    let t51870 = 595.0_f64 / 10368.0_f64 * t51869;
    let t51877 = t13987 * t894;
    let t51881 = t13953 * t13855;
    let t51890 = t8599 * t353 * t1193 * t2182;
    (t51829, t51864, t51870, t51877, t51881, t51890)
}
