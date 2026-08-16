//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2857/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2857(t12904: f64, t3708: f64, t11262: f64, t1247: f64, t3590: f64, t3610: f64, t3612: f64, t1231: f64, t12898: f64, t3651: f64, t3655: f64, t43813: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44270 = t3708 * t12904;
    let t44273 = t1247 * t11262 * t3590;
    let t44276 = t3610 * t11262 * t3612;
    let t44291 = t1231 * t12898;
    let t44293 = t3651 * t3655;
    let t44307 = 0.86419753086419753087e-1_f64 * t43813;
    (t44270, t44273, t44276, t44291, t44293, t44307)
}
