//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta369 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1912;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta369(t12916: f64, t3722: f64, t3718: f64, t3172: f64, t3590: f64, t1247: f64, t3612: f64, t3610: f64, t1260: f64, t3666: f64, t3713: f64, t3711: f64, t127: f64, t3661: f64, t371: f64, t1235: f64, t12640: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12917, t12918, t12941, t12942, t12948, t12949, t12956) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1912(t12916, t3722, t3718, t3172, t3590, t1247, t3612, t3610, t1260, t3666);
        let (t12959, t12960, t12963, t12964, t12966) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1913(t3172, t3713, t3711, t127, t3661, t371, t1235, t12640, t225);
    (t12917, t12918, t12941, t12942, t12948, t12949, t12956, t12959, t12960, t12963, t12964, t12966)
}
