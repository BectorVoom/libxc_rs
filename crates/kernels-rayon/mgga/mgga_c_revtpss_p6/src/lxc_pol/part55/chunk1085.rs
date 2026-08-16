//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1085/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1085(t246: f64, t5704: f64, t32289: f64, t8591: f64, t1916: f64, t8614: f64, t1518: f64, t32374: f64, t572: f64, t7937: f64, t8698: f64, t8108: f64, t8717: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33969 = t246 * t5704;
    let t33970 = t32289 * t33969;
    let t33971 = t8591 * t33970;
    let t34010 = t1916 * t8614;
    let t34011 = 3.0_f64 * t34010;
    let t34012 = t32374 * t1518;
    let t34013 = t572 * t34012;
    let t34014 = 6.0_f64 * t34013;
    let t34017 = t8698 * t7937;
    let t34018 = t8108 * t8717;
    (t33969, t33970, t33971, t34011, t34012, t34014, t34017, t34018)
}
