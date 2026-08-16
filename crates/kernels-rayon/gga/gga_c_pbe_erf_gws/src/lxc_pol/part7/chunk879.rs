//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 879/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk879(t16780: f64, t5175: f64, t590: f64, t418: f64, t5177: f64, t572: f64, t587: f64, t1820: f64, t1866: f64, t562: f64, t610: f64, t7703: f64) -> (f64, f64, f64) {
    let t16781 = 16.0_f64 / 15.0_f64 * t16780;
    let t16782 = t590 * t5175;
    let t16787 = 32.0_f64 / 15.0_f64 * t587 * t16782 * t5177 * t572 * t418;
    let t16792 = 32.0_f64 / 5.0_f64 * t1820 * t7703 * t610 * t1866 * t562;
    (t16781, t16787, t16792)
}
