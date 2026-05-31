//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1162/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1162<F: Float>(t5: F, t95254: F, t95281: F, t95313: F, t95343: F, t117: F, t116: F, t26209: F, t10194: F, t1310: F, t13207: F, t13435: F, t13625: F, t18163: F, t2052: F, t2056: F, t2089: F, t2322: F, t2328: F, t2331: F, t25082: F, t26153: F, t26210: F, t26223: F, t26377: F, t26396: F, t26399: F, t26406: F, t26674: F, t33183: F, t3813: F, t4254: F, t49693: F, t49856: F, t508: F, t651: F, t670: F, t671: F, t7235: F, t7367: F, t7373: F, t7474: F, t95088: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t95346 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t95254 + t95281 + t95313 + t95343);
    let t95347 = t95346 * t117;
    let t95357 = t26209 * t116;
    let t95362 = -F::cast_from(12.0_f64) * t2322 * t26396 - F::cast_from(12.0_f64) * t4254 * t26396 - F::cast_from(6.0_f64) * t651 * t3813 * t7373 - F::cast_from(6.0_f64) * t10194 * t2089 - F::cast_from(6.0_f64) * t2328 * t7474 + F::cast_from(3.0_f64) * t7235 * t26377 - F::cast_from(6.0_f64) * t49693 * t2056 - F::cast_from(2.0_f64) * t49856 * t2056 - F::cast_from(6.0_f64) * t18163 * t7367 - F::cast_from(18.0_f64) * t95088 * t26406 - t2052 * t13207 - F::cast_from(12.0_f64) * t2322 * t26223 - F::cast_from(6.0_f64) * t651 * t26674 * t670 - F::cast_from(12.0_f64) * t13435 * t7367 - t95347 * t508 - F::cast_from(3.0_f64) * t26210 * t1310 - F::cast_from(18.0_f64) * t25082 * t33183 * t13625 - F::cast_from(6.0_f64) * t651 * t1310 * t26153 - F::cast_from(6.0_f64) * t95357 * t671 - F::cast_from(12.0_f64) * t26399 * t2331;
    (t95347, t95357, t95362)
}
