//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 857/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk857(t16499: f64, t16521: f64, t16549: f64, t16566: f64, t106: f64, t1299: f64, t13300: f64, t16443: f64, t16456: f64, t16460: f64, t167: f64, t3454: f64, t3461: f64, t4675: f64, t4723: f64, t670: f64, t6977: f64, t9794: f64) -> (f64, f64) {
    let t16568 = t16499 + t16521 + t16549 + t16566;
    let t16572 = 0.27818116767324025134e1_f64 * t106 * t16443 * t167 - 0.83454350301972075402e1_f64 * t106 * t13300 * t1299 + 0.16690870060394415081e2_f64 * t106 * t9794 * t4675 - 0.83454350301972075402e1_f64 * t106 * t3454 * t4723 - 0.1669087006039441508e2_f64 * t106 * t6977 * t16456 + 0.16690870060394415081e2_f64 * t3461 * t16460 - 0.27818116767324025134e1_f64 * t106 * t670 * t16568;
    (t16568, t16572)
}
