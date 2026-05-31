//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3215/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3215<F: Float>(t5: F, t60692: F, t61007: F, t117: F, t10416: F, t1310: F, t13425: F, t13429: F, t13435: F, t1502: F, t1518: F, t18153: F, t18220: F, t18242: F, t1843: F, t21658: F, t21814: F, t2320: F, t2322: F, t3813: F, t4246: F, t508: F, t5517: F, t5877: F, t5921: F, t60650: F, t60656: F, t649: F, t651: F, t6765: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t61009 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t60692 + t61007);
    let t61010 = t61009 * t117;
    let t61014 = -F::cast_from(4.0_f64) * t1518 * t18153 * t651 - F::cast_from(2.0_f64) * t10416 * t5921 - F::cast_from(4.0_f64) * t1310 * t18220 - F::cast_from(2.0_f64) * t1310 * t21814 - F::cast_from(2.0_f64) * t13425 * t1843 - F::cast_from(4.0_f64) * t13429 * t1843 - F::cast_from(4.0_f64) * t13435 * t5921 - F::cast_from(2.0_f64) * t1502 * t18153 - F::cast_from(4.0_f64) * t18242 * t2322 - F::cast_from(2.0_f64) * t21658 * t649 - t2320 * t6765 - t3813 * t5877 - F::cast_from(4.0_f64) * t4246 * t5517 - F::cast_from(2.0_f64) * t508 * t60650 - F::cast_from(2.0_f64) * t508 * t60656 - t508 * t61010;
    (t61010, t61014)
}
