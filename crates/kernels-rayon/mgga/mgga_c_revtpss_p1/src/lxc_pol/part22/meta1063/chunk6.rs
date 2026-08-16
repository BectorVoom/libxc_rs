//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3810/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3810(t33: f64, t265: f64, t502: f64, t63193: f64, t68629: f64, t73260: f64, t73266: f64, t73270: f64, t73277: f64, t73283: f64, t73285: f64, t73286: f64, t1113: f64, t1304: f64, t13312: f64, t1469: f64, t15083: f64, t1711: f64, t18140: f64, t18281: f64, t1837: f64, t18884: f64, t20256: f64, t21645: f64, t2258: f64, t2838: f64, t3351: f64, t3805: f64, t4186: f64, t504: f64, t51835: f64, t5509: f64, t57: f64, t5825: f64, t606: f64, t60754: f64, t6084: f64, t63202: f64, t63204: f64, t63206: f64, t6416: f64, t6757: f64, t895: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t73290 = piecewise3(t503, t68629 + t73260 + t73266 + t73270 + t73277 + t73283 + t73285 + t73286, t63193);
    let t73306 = piecewise3(t400, t63193 * t33 / 2.0_f64 + t18884 * t1113 + t6084 * t3351 / 2.0_f64 + t15083 * t1711 - t63202 - t63204 + t63206 + t2838 * t6416 / 2.0_f64 + t895 * t20256 - t51835, t73290 * t57 / 2.0_f64 - t21645 * t606 - t6757 * t2258 / 2.0_f64 - t18140 * t1469 - 2.0_f64 * t5509 * t4186 - t1837 * t13312 - t3805 * t5825 / 2.0_f64 - t1304 * t18281 - t504 * t60754 / 2.0_f64);
    t73306
}
