//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3715/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3715<F: Float>(t3584: F, t6587: F, t21192: F, t3647: F, t1042: F, t12956: F, t1715: F, t17261: F, t20876: F, t20880: F, t21246: F, t247: F, t3711: F, t3719: F, t5384: F, t57125: F, t57303: F, t68669: F, t70394: F, t70403: F, t70405: F, t70411: F, t70413: F) -> (F, F) {
    let t70422 = t6587 * t3584;
    let t70427 = t3647 * t21192;
    let t70429 = F::cast_from(0.57165357490759649296e-3_f64) * t12956 * t20876 + F::cast_from(0.19055119163586549765e-3_f64) * t70394 + F::cast_from(0.57165357490759649296e-3_f64) * t12956 * t20880 + F::cast_from(0.28582678745379824648e-3_f64) * t3711 * t1042 * t57303 * t1715 - F::cast_from(0.19055119163586549765e-3_f64) * t57125 + F::cast_from(0.20325460441158986416e-2_f64) * t70403 + F::cast_from(0.6351706387862183255e-4_f64) * t70405 + F::cast_from(0.85748036236139473944e-3_f64) * t17261 * t21246 + F::cast_from(0.11433071498151929859e-2_f64) * t70411 + F::cast_from(0.85748036236139473944e-3_f64) * t5384 * t247 * t3719 * t70413 + F::cast_from(0.85748036236139473944e-3_f64) * t5384 * t247 * t3719 * t68669 + F::cast_from(0.42874018118069736972e-3_f64) * t5384 * t247 * t3719 * t70422 - F::cast_from(0.3811023832717309953e-3_f64) * t70427;
    (t70422, t70429)
}
