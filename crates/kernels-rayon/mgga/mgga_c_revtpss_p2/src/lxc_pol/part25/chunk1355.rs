//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1355/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1355(t2014: f64, t25089: f64, t25190: f64, t28167: f64, t49616: f64, t8717: f64, t1450: f64, t2033: f64, t9400: f64, t10192: f64, t1310: f64, t13207: f64, t1932: f64, t2011: f64, t2328: f64, t2371: f64, t2372: f64, t25096: f64, t25800: f64, t28025: f64, t3813: f64, t4151: f64, t508: f64, t651: f64, t670: f64, t6983: f64, t7221: f64, t7231: f64, t94947: f64, t95073: f64, t95075: f64, t95081: f64, t95083: f64, t95085: f64, t95087: f64, t95090: f64) -> f64 {
    let t95096 = 9.0_f64 * t2014 * t25190 * t25089;
    let t95104 = 18.0_f64 * t28167 * t8717 * t49616;
    let t95108 = 6.0_f64 * t2014 * t9400 * t2033 * t1450;
    let t95117 = -6.0_f64 * t2371 * t651 * t7221 - 6.0_f64 * t25800 * t651 * t670 + t10192 * t2011 - 6.0_f64 * t1310 * t25096 - t13207 * t1932 - 6.0_f64 * t2328 * t7221 - 6.0_f64 * t2372 * t28025 - 3.0_f64 * t3813 * t6983 + 3.0_f64 * t4151 * t7231 - 6.0_f64 * t508 * t94947 - t95073 - t95075 + t95081 - t95083 - t95085 - t95087 - t95090 + t95096 - t95104 + t95108;
    t95117
}
