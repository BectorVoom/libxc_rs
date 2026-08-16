//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1303/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1303(t10373: f64, t3724: f64, t11619: f64, t2493: f64, t3209: f64, t1054: f64, t2316: f64, t3723: f64, t10105: f64, t11620: f64, t2255: f64, t2674: f64, t996: f64) -> (f64, f64, f64, f64, f64) {
    let t36022 = t10373 * t3724;
    let t36025 = t3209 * t11619 * t2493;
    let t36028 = t1054 * t3723 * t2316;
    let t36030 = t10105 * t11620;
    let t36034 = t996 * t2674 * t3723 * t2255;
    (t36022, t36025, t36028, t36030, t36034)
}
