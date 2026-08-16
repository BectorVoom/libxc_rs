//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 872/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk872(t1364: f64, t26064: f64, t26050: f64, t7289: f64, t25304: f64, t7283: f64, t25946: f64, t25949: f64, t786: f64, t7286: f64, t213: f64, t7274: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26065 = t26064 * t1364;
    let t26067 = t7289 * t26050;
    let t26069 = t25304 * t7283;
    let t26071 = 0.22849835011101738147e-2_f64 * t26069 * t25946;
    let t26072 = t786 * t25949;
    let t26073 = t26072 * t7286;
    let t26084 = t213 * t7274;
    (t26065, t26067, t26069, t26071, t26072, t26073, t26084)
}
