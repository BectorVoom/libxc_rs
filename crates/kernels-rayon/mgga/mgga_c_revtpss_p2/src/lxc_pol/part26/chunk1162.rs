//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1162/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1162(t5: f64, t95254: f64, t95281: f64, t95313: f64, t95343: f64, t117: f64, t116: f64, t26209: f64, t10194: f64, t1310: f64, t13207: f64, t13435: f64, t13625: f64, t18163: f64, t2052: f64, t2056: f64, t2089: f64, t2322: f64, t2328: f64, t2331: f64, t25082: f64, t26153: f64, t26210: f64, t26223: f64, t26377: f64, t26396: f64, t26399: f64, t26406: f64, t26674: f64, t33183: f64, t3813: f64, t4254: f64, t49693: f64, t49856: f64, t508: f64, t651: f64, t670: f64, t671: f64, t7235: f64, t7367: f64, t7373: f64, t7474: f64, t95088: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t95346 = piecewise3(t8, 0.0_f64, t95254 + t95281 + t95313 + t95343);
    let t95347 = t95346 * t117;
    let t95357 = t26209 * t116;
    let t95362 = -12.0_f64 * t2322 * t26396 - 12.0_f64 * t4254 * t26396 - 6.0_f64 * t651 * t3813 * t7373 - 6.0_f64 * t10194 * t2089 - 6.0_f64 * t2328 * t7474 + 3.0_f64 * t7235 * t26377 - 6.0_f64 * t49693 * t2056 - 2.0_f64 * t49856 * t2056 - 6.0_f64 * t18163 * t7367 - 18.0_f64 * t95088 * t26406 - t2052 * t13207 - 12.0_f64 * t2322 * t26223 - 6.0_f64 * t651 * t26674 * t670 - 12.0_f64 * t13435 * t7367 - t95347 * t508 - 3.0_f64 * t26210 * t1310 - 18.0_f64 * t25082 * t33183 * t13625 - 6.0_f64 * t651 * t1310 * t26153 - 6.0_f64 * t95357 * t671 - 12.0_f64 * t26399 * t2331;
    (t95347, t95357, t95362)
}
