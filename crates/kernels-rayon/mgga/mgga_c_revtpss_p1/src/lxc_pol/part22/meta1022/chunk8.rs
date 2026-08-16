//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3569/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3569(t30: f64, t265: f64, t393: f64, t63193: f64, t63587: f64, t63629: f64, t63671: f64, t63899: f64, t63938: f64, t64513: f64, t64532: f64, t68211: f64, t1106: f64, t13312: f64, t1468: f64, t1469: f64, t15083: f64, t16618: f64, t1704: f64, t18280: f64, t18281: f64, t18884: f64, t20236: f64, t2257: f64, t2258: f64, t2838: f64, t3340: f64, t395: f64, t4186: f64, t45: f64, t5028: f64, t51835: f64, t5824: f64, t5825: f64, t605: f64, t606: f64, t60754: f64, t6084: f64, t63202: f64, t63204: f64, t63206: f64, t6405: f64, t895: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t68215 = piecewise3(t394, t63587 + t63629 + t63671 + t63899 + t63938 + t64513 + t64532 + t68211, t63193);
    let t68231 = piecewise3(t120, t63193 * t30 / 2.0_f64 + t18884 * t605 + t6084 * t2257 / 2.0_f64 + t15083 * t1468 + t63202 + t63204 - t63206 + t2838 * t5824 / 2.0_f64 + t895 * t18280 + t51835, t68215 * t45 / 2.0_f64 + t20236 * t606 + t6405 * t2258 / 2.0_f64 + t16618 * t1469 + 2.0_f64 * t5028 * t4186 + t1704 * t13312 + t3340 * t5825 / 2.0_f64 + t1106 * t18281 + t395 * t60754 / 2.0_f64);
    t68231
}
