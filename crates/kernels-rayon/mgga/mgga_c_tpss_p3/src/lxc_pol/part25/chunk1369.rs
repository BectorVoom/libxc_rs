//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1369/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1369(t30: f64, t259: f64, t379: f64, t72363: f64, t72411: f64, t1289: f64, t13335: f64, t1819: f64, t20577: f64, t21702: f64, t3431: f64, t45: f64, t4579: f64, t581: f64, t5870: f64, t6374: f64, t72203: f64, t72242: f64, t72277: f64, t72317: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t72412 = t72363 + t72411;
    let t72413 = piecewise3(t380, 0.0_f64, t72412);
    let t72425 = piecewise3(t120, t72203 + t72242 + t72277 + t72317, t72413 * t45 / 2.0_f64 + t21702 * t581 / 2.0_f64 + t20577 * t1289 + t6374 * t3431 + t5870 * t4579 / 2.0_f64 + t1819 * t13335 / 2.0_f64);
    (t72412, t72425)
}
