//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3711/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3711<F: Float>(t17401: F, t17620: F, t17728: F, t489: F, t5219: F, t1256: F, t21335: F, t20900: F, t3153: F, t3609: F, t69692: F, t1042: F, t12787: F, t12956: F, t13396: F, t16719: F, t17760: F, t17786: F, t20825: F, t21017: F, t3613: F, t3650: F, t3711: F, t3720: F, t484: F, t5302: F, t5331: F, t5333: F, t57005: F, t57075: F, t57077: F, t57094: F, t6594: F, t69763: F) -> (F, F, F) {
    let t70300 = t17401 * t17620;
    let t70303 = t5219 * t489 * t17728;
    let t70306 = t21335 * t1256;
    let t70311 = t20900 * t3153;
    let t70319 = t69692 * t3609;
    let t70328 = -F::cast_from(0.3811023832717309953e-3_f64) * t57075 + F::cast_from(0.10162730220579493208e-2_f64) * t57077 + F::cast_from(0.57165357490759649297e-2_f64) * t57005 * t12787 * t16719 * t13396 - F::cast_from(0.57165357490759649296e-3_f64) * t70300 - F::cast_from(0.95275595817932748826e-3_f64) * t70303 * t17760 + F::cast_from(0.28582678745379824648e-3_f64) * t70306 + F::cast_from(0.72409452821628889107e-2_f64) * t3650 * t6594 * t484 - F::cast_from(0.42874018118069736972e-3_f64) * t5331 * t3720 * t70311 * t5333 + F::cast_from(0.22866142996303859718e-2_f64) * t21017 * t17786 + F::cast_from(0.1270341277572436651e-3_f64) * t57094 - F::cast_from(0.21437009059034868486e-3_f64) * t70319 * t3613 - F::cast_from(0.47637797908966374414e-3_f64) * t12956 * t20825 - F::cast_from(0.23818898954483187207e-3_f64) * t3711 * t1042 * t5302 * t69763;
    (t70303, t70311, t70328)
}
