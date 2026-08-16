//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1157/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1157(t30: f64, t126434: f64, t129301: f64, t1469: f64, t32785: f64, t34388: f64, t4186: f64, t45: f64, t606: f64, t8752: f64, t28184: f64, t8764: f64, t2322: f64, t34428: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t129308 = piecewise3(t120, t126434, t129301 * t45 / 2.0_f64 + t32785 * t1469 / 2.0_f64 + t34388 * t606 / 2.0_f64 + t8752 * t4186 / 2.0_f64);
    let t129312 = t8764 * t28184;
    let t129314 = t2322 * t34428;
    (t129308, t129312, t129314)
}
