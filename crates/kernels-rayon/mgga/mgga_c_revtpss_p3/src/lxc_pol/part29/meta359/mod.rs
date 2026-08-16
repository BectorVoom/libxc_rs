//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta359 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1292;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1293;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta359(t12916: f64, t3722: f64, t3718: f64, t3172: f64, t3590: f64, t1247: f64, t3612: f64, t3610: f64, t1260: f64, t3666: f64, t3713: f64, t3711: f64, t127: f64, t3661: f64, t371: f64, t1235: f64, t12640: f64, t225: f64, t12657: f64, t480: f64, t3667: f64, t3678: f64, t1236: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12918, t12942, t12949, t12956, t12960) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1292(t12916, t3722, t3718, t3172, t3590, t1247, t3612, t3610, t1260, t3666, t3713, t3711);
        let (t12964, t12966, t12975, t12976, t12979, t12984) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1293(t127, t3661, t371, t1235, t12640, t225, t12657, t480, t3667, t3678, t1236, t676);
    (t12918, t12942, t12949, t12956, t12960, t12964, t12966, t12975, t12976, t12979, t12984)
}
