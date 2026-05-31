//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 805/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk805<F: Float>(t21522: F, t21551: F, t21696: F, t21768: F, t21717: F, t258: F, t1137: F, t1173: F, t21123: F, t21125: F, t21464: F, t21500: F, t21532: F, t21540: F, t21548: F, t21640: F, t21688: F, t247: F, t263: F, t4915: F, t5059: F, t5179: F) -> (F, F, F) {
    let t21770 = t21522 + t21551 + t21696 + t21768;
    let t21772 = t21717 * t258;
    let t21780 = -F::cast_from(3.0_f64) * t1137 * t5179 - F::cast_from(3.0_f64) * t1173 * t4915 - F::cast_from(3.0_f64) * t1173 * t5059 - t21123 * t263 - F::cast_from(2.0_f64) * t21125 * t263 - t21464 * t263 - t21770 * t247 + F::cast_from(12.0_f64) * t21500 - F::cast_from(12.0_f64) * t21532 - F::cast_from(6.0_f64) * t21540 - F::cast_from(6.0_f64) * t21548 - F::cast_from(2.0_f64) * t21640 + F::cast_from(12.0_f64) * t21688 + F::cast_from(2.0_f64) * t21772;
    (t21770, t21772, t21780)
}
