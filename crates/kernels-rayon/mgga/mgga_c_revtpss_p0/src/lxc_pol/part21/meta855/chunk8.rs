//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3243/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3243(t33: f64, t265: f64, t502: f64, t51814: f64, t56291: f64, t60124: f64, t60130: f64, t60139: f64, t60142: f64, t60143: f64, t60147: f64, t60155: f64, t10326: f64, t11095: f64, t1113: f64, t1304: f64, t13196: f64, t13312: f64, t1469: f64, t15083: f64, t1587: f64, t1711: f64, t18140: f64, t1837: f64, t2258: f64, t3351: f64, t3805: f64, t4186: f64, t4560: f64, t49889: f64, t504: f64, t51827: f64, t51829: f64, t51831: f64, t51833: f64, t51835: f64, t5509: f64, t57: f64, t606: f64, t9357: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t60159 = piecewise3(t503, t56291 + t60124 + t60130 + t60139 + t60142 + t60143 + t60147 + t60155, t51814);
    let t60177 = piecewise3(t400, t51814 * t33 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t15083 * t1113 + 3.0_f64 / 2.0_f64 * t4560 * t3351 + t1587 * t9357 / 2.0_f64 + t11095 * t1711 / 2.0_f64 - t51827 - t51829 + t51831 + t51833 - t51835, t60159 * t57 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t18140 * t606 - 3.0_f64 / 2.0_f64 * t5509 * t2258 - t1837 * t10326 / 2.0_f64 - t13196 * t1469 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t3805 * t4186 - 3.0_f64 / 2.0_f64 * t1304 * t13312 - t504 * t49889 / 2.0_f64);
    t60177
}
