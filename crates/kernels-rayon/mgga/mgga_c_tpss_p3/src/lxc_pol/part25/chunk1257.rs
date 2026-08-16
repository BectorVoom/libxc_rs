//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1257/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1257(t30: f64, t259: f64, t379: f64, t21701: f64, t1289: f64, t1819: f64, t21677: f64, t45: f64, t4579: f64, t6374: f64, t1812: f64, t21485: f64, t1497: f64, t1692: f64, t18728: f64, t18812: f64, t20514: f64, t21492: f64, t21495: f64, t21499: f64, t21510: f64, t21513: f64, t21516: f64, t21659: f64, t2439: f64, t33: f64, t3552: f64, t5059: f64, t5853: f64, t6207: f64, t6214: f64, t6354: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t21702 = piecewise3(t380, 0.0_f64, t21701);
    let t21709 = piecewise3(t120, t21677, t21702 * t45 / 2.0_f64 + t6374 * t1289 + t1819 * t4579 / 2.0_f64);
    let t21710 = t1812 * t21485;
    let t21741 = 3.0_f64 * t3552 * t21710 + 3.0_f64 * t2439 * t6354 * t6207 - 3.0_f64 * t18728 * t21492 + 3.0_f64 * t2439 * t1812 * t21495 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t21499 + t1692 * t21659 * t33 / 2.0_f64 - t1692 * t20514 * t6214 + t1692 * t6354 * t1497 + t1692 * t18812 * t21510 - t1692 * t5853 * t21513 - t1692 * t5853 * t21516 / 2.0_f64 + t1692 * t1812 * t5059 / 2.0_f64;
    (t21702, t21709, t21710, t21741)
}
