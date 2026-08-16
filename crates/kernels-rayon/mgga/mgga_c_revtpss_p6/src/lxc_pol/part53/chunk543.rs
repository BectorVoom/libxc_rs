//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 543/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk543(t225: f64, t4376: f64, t4407: f64, t227: f64, t73: f64, t1544: f64, t853: f64, t775: f64, t4343: f64, t832: f64, t1553: f64, t1555: f64, t229: f64, t830: f64, t833: f64) -> f64 {
    let t4409 = (t4376 + t4407) * t225;
    let t4415 = t227 * t73;
    let t4416 = t853 * t1544;
    let t4417 = t4416 * t775;
    let t4420 = t832 * t4343;
    let t4423 = 3.0_f64 * t1553 * t833 + 3.0_f64 * t1555 * t830 + 3.0_f64 * t227 * t4420 - t229 * t4409 - 12.0_f64 * t4415 * t4417;
    t4423
}
