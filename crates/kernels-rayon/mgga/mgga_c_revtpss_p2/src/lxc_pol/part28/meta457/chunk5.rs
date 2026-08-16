//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1748/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1748(t33: f64, t265: f64, t502: f64, t15083: f64, t18127: f64, t18138: f64, t1113: f64, t1304: f64, t13312: f64, t1469: f64, t15093: f64, t15094: f64, t15096: f64, t1587: f64, t1711: f64, t1837: f64, t2258: f64, t2838: f64, t3351: f64, t3805: f64, t4186: f64, t4560: f64, t504: f64, t5509: f64, t57: f64, t606: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t18140 = piecewise3(t503, t18127 + t18138, t15083);
    let t18152 = piecewise3(t400, t15083 * t33 / 2.0_f64 + t4560 * t1113 + t1587 * t3351 / 2.0_f64 + t2838 * t1711 / 2.0_f64 - t15093 - t15094 + t15096, t18140 * t57 / 2.0_f64 - t5509 * t606 - t1837 * t2258 / 2.0_f64 - t3805 * t1469 / 2.0_f64 - t1304 * t4186 - t504 * t13312 / 2.0_f64);
    t18152
}
