//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3711/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3711(t17401: f64, t17620: f64, t17728: f64, t489: f64, t5219: f64, t1256: f64, t21335: f64, t20900: f64, t3153: f64, t3609: f64, t69692: f64, t1042: f64, t12787: f64, t12956: f64, t13396: f64, t16719: f64, t17760: f64, t17786: f64, t20825: f64, t21017: f64, t3613: f64, t3650: f64, t3711: f64, t3720: f64, t484: f64, t5302: f64, t5331: f64, t5333: f64, t57005: f64, t57075: f64, t57077: f64, t57094: f64, t6594: f64, t69763: f64) -> (f64, f64, f64) {
    let t70300 = t17401 * t17620;
    let t70303 = t5219 * t489 * t17728;
    let t70306 = t21335 * t1256;
    let t70311 = t20900 * t3153;
    let t70319 = t69692 * t3609;
    let t70328 = -0.3811023832717309953e-3_f64 * t57075 + 0.10162730220579493208e-2_f64 * t57077 + 0.57165357490759649297e-2_f64 * t57005 * t12787 * t16719 * t13396 - 0.57165357490759649296e-3_f64 * t70300 - 0.95275595817932748826e-3_f64 * t70303 * t17760 + 0.28582678745379824648e-3_f64 * t70306 + 0.72409452821628889107e-2_f64 * t3650 * t6594 * t484 - 0.42874018118069736972e-3_f64 * t5331 * t3720 * t70311 * t5333 + 0.22866142996303859718e-2_f64 * t21017 * t17786 + 0.1270341277572436651e-3_f64 * t57094 - 0.21437009059034868486e-3_f64 * t70319 * t3613 - 0.47637797908966374414e-3_f64 * t12956 * t20825 - 0.23818898954483187207e-3_f64 * t3711 * t1042 * t5302 * t69763;
    (t70303, t70311, t70328)
}
