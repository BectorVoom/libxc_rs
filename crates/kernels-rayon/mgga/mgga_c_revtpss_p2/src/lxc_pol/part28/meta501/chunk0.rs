//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1888/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1888(t25304: f64, t7283: f64, t25946: f64, t25949: f64, t786: f64, t7286: f64, t225: f64, t26034: f64, t1426: f64, t3999: f64, t26044: f64, t4003: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26069 = t25304 * t7283;
    let t26071 = 0.22849835011101738147e-2_f64 * t26069 * t25946;
    let t26072 = t786 * t25949;
    let t26073 = t26072 * t7286;
    let t26075 = t26034 * t225;
    let t26079 = t1426 * t3999;
    let t26080 = t26044 * t4003;
    (t26069, t26071, t26072, t26073, t26075, t26079, t26080)
}
