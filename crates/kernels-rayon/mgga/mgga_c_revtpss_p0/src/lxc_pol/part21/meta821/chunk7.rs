//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3045/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3045(t30: f64, t265: f64, t393: f64, t51814: f64, t52167: f64, t52197: f64, t52227: f64, t52870: f64, t52883: f64, t52906: f64, t52924: f64, t56115: f64, t10326: f64, t1106: f64, t11095: f64, t12201: f64, t13312: f64, t1468: f64, t1469: f64, t15083: f64, t1587: f64, t16618: f64, t1704: f64, t2257: f64, t2258: f64, t3340: f64, t395: f64, t4186: f64, t45: f64, t4560: f64, t49889: f64, t5028: f64, t51827: f64, t51829: f64, t51831: f64, t51833: f64, t51835: f64, t605: f64, t606: f64, t9344: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t56119 = piecewise3(t394, t52167 + t52197 + t52227 + t52870 + t52883 + t52906 + t52924 + t56115, t51814);
    let t56137 = piecewise3(t120, t51814 * t30 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t15083 * t605 + 3.0_f64 / 2.0_f64 * t4560 * t2257 + t1587 * t9344 / 2.0_f64 + t11095 * t1468 / 2.0_f64 + t51827 + t51829 - t51831 - t51833 + t51835, t56119 * t45 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t16618 * t606 + 3.0_f64 / 2.0_f64 * t5028 * t2258 + t1704 * t10326 / 2.0_f64 + t12201 * t1469 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t3340 * t4186 + 3.0_f64 / 2.0_f64 * t1106 * t13312 + t395 * t49889 / 2.0_f64);
    t56137
}
