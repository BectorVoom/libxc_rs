//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1015/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1015(t25: f64, t265: f64, t394: f64, t128193: f64, t128093: f64, t128134: f64, t1409: f64, t33513: f64, t40: f64, t5398: f64, t8580: f64, t100688: f64, t101226: f64, t101840: f64, t126992: f64, t127017: f64, t127030: f64, t128076: f64, t128080: f64, t128101: f64, t1877: f64, t24191: f64, t2522: f64, t25927: f64, t26744: f64, t26756: f64, t28: f64, t28764: f64, t28778: f64, t33476: f64, t33483: f64, t33537: f64, t33539: f64, t4314: f64, t5966: f64, t7114: f64, t8566: f64, t8586: f64, t89992: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t128194 = piecewise3(t395, 0.0_f64, t128193);
    let t128201 = piecewise3(t115, t128093 + t128134, t128194 * t40 / 2.0_f64 + t33513 * t1409 + t8580 * t5398 / 2.0_f64);
    let t128239 = 2.0_f64 * t26756 * t100688 * t33483 + 2.0_f64 * t26756 * t127030 + t26756 * t25927 * t128101 + t1877 * t8566 * t5966 / 2.0_f64 + t1877 * t128076 * t28 / 2.0_f64 - 3.0_f64 * t24191 * t89992 * t33476 + 6.0_f64 * t24191 * t25927 * t128080 + 3.0_f64 / 2.0_f64 * t2522 * t8566 * t28778 + 3.0_f64 * t4314 * t8566 * t28764 - t1877 * t7114 * t127017 - t1877 * t26744 * t33539 + 2.0_f64 * t101840 * t33537 - t1877 * t7114 * t126992 / 2.0_f64 - t1877 * t101226 * t8586 / 2.0_f64;
    (t128201, t128239)
}
