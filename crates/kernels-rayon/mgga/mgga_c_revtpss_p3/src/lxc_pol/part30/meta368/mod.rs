//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1395;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta368(t12916: f64, t3722: f64, t3718: f64, t3172: f64, t3590: f64, t1247: f64, t3612: f64, t3610: f64, t1260: f64, t3666: f64, t3713: f64, t3711: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12917, t12918, t12941, t12942, t12948, t12949, t12956, t12959, t12960) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1395(t12916, t3722, t3718, t3172, t3590, t1247, t3612, t3610, t1260, t3666, t3713, t3711);
    (t12917, t12918, t12941, t12942, t12948, t12949, t12956, t12959, t12960)
}
