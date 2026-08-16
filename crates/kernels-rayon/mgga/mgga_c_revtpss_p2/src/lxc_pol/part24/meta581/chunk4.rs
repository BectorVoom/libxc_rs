//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1809/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1809(t33: f64, t265: f64, t502: f64, t87990: f64, t91754: f64, t91758: f64, t91765: f64, t91774: f64, t1469: f64, t1587: f64, t1711: f64, t1837: f64, t22671: f64, t22783: f64, t23436: f64, t25032: f64, t504: f64, t57: f64, t5825: f64, t6084: f64, t6416: f64, t6757: f64, t87126: f64, t89780: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t91777 = piecewise3(t503, t91754 + t91758 + t91765 + t91774, t87990);
    let t91789 = piecewise3(t400, t87990 * t33 / 2.0_f64 + 2.0_f64 * t23436 * t1711 + 3.0_f64 * t6084 * t6416 + 2.0_f64 * t1587 * t22783 + t265 * t89780 / 2.0_f64, t91777 * t57 / 2.0_f64 - 2.0_f64 * t25032 * t1469 - 3.0_f64 * t6757 * t5825 - 2.0_f64 * t1837 * t22671 - t504 * t87126 / 2.0_f64);
    t91789
}
