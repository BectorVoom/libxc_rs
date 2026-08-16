//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta360 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1294;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1295;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta360(t1235: f64, t12984: f64, t12627: f64, t225: f64, t127: f64, t3672: f64, t371: f64, t3671: f64, t140: f64, t3693: f64, t1222: f64, t1226: f64, t697: f64, t3688: f64, t3700: f64, t3367: f64, t404: f64, t1242: f64, t3603: f64, t471: f64, t1032: f64, t3552: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12985, t12987, t12996, t12999, t13011) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1294(t1235, t12984, t12627, t225, t127, t3672, t371, t3671, t140, t3693, t1222, t1226, t697);
        let (t13012, t13015, t13018, t13026, t13038, t13045, t13068) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1295(t1222, t13011, t140, t3688, t3700, t3367, t404, t1242, t3603, t471, t1032, t3552);
    (t12985, t12987, t12996, t12999, t13012, t13015, t13018, t13026, t13038, t13045, t13068)
}
