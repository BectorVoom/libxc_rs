//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 982/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk982(t33384: f64, t33552: f64, t3: f64, t1461: f64, t32365: f64, t32368: f64, t32371: f64, t32373: f64, t32377: f64, t32901: f64, t32903: f64, t32905: f64, t573: f64, t8616: f64, t8975: f64, param_d: f64) -> (f64, f64, f64, f64) {
    let t33553 = t33384 + t33552;
    let t33554 = t3 * t33553;
    let t33565 = param_d * t33553;
    let t33572 = 3.0_f64 * t1461 * t8975 + t33565 * t573 + t32365 + t32368 + t32371 + t32373 + t32377 + 6.0_f64 * t32901 + 12.0_f64 * t32903 + 6.0_f64 * t32905 + t8616;
    (t33553, t33554, t33565, t33572)
}
