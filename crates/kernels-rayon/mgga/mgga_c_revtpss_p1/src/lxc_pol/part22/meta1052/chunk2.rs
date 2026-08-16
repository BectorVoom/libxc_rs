//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3715/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3715(t3584: f64, t6587: f64, t21192: f64, t3647: f64, t1042: f64, t12956: f64, t1715: f64, t17261: f64, t20876: f64, t20880: f64, t21246: f64, t247: f64, t3711: f64, t3719: f64, t5384: f64, t57125: f64, t57303: f64, t68669: f64, t70394: f64, t70403: f64, t70405: f64, t70411: f64, t70413: f64) -> (f64, f64) {
    let t70422 = t6587 * t3584;
    let t70427 = t3647 * t21192;
    let t70429 = 0.57165357490759649296e-3_f64 * t12956 * t20876 + 0.19055119163586549765e-3_f64 * t70394 + 0.57165357490759649296e-3_f64 * t12956 * t20880 + 0.28582678745379824648e-3_f64 * t3711 * t1042 * t57303 * t1715 - 0.19055119163586549765e-3_f64 * t57125 + 0.20325460441158986416e-2_f64 * t70403 + 0.6351706387862183255e-4_f64 * t70405 + 0.85748036236139473944e-3_f64 * t17261 * t21246 + 0.11433071498151929859e-2_f64 * t70411 + 0.85748036236139473944e-3_f64 * t5384 * t247 * t3719 * t70413 + 0.85748036236139473944e-3_f64 * t5384 * t247 * t3719 * t68669 + 0.42874018118069736972e-3_f64 * t5384 * t247 * t3719 * t70422 - 0.3811023832717309953e-3_f64 * t70427;
    (t70422, t70429)
}
