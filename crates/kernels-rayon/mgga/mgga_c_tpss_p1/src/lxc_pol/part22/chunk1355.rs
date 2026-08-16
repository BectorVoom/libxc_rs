//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1355/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1355(t30: f64, t259: f64, t379: f64, t66704: f64, t66750: f64, t10353: f64, t1289: f64, t1819: f64, t18848: f64, t1992: f64, t20577: f64, t3431: f64, t45: f64, t581: f64, t5870: f64, t6374: f64, t66266: f64, t66302: f64, t66618: f64, t66656: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t66751 = t66704 + t66750;
    let t66752 = piecewise3(t380, 0.0_f64, t66751);
    let t66764 = piecewise3(t120, t66266 + t66302 + t66618 + t66656, t66752 * t45 / 2.0_f64 + t20577 * t581 + t6374 * t1992 / 2.0_f64 + t18848 * t1289 / 2.0_f64 + t5870 * t3431 + t1819 * t10353 / 2.0_f64);
    (t66751, t66764)
}
