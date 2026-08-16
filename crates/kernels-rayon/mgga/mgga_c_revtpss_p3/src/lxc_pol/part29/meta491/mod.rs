//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta491 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1780;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1781;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1782;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta491(t233: f64, t28340: f64, t1957: f64, t2061: f64, t231: f64, t4423: f64, t7076: f64, t25317: f64, t8006: f64, t886: f64, t4533: f64, t7071: f64, t27213: f64, t7407: f64, t1956: f64, t26508: f64, t26521: f64, t26522: f64, t26529: f64, t26534: f64, t26536: f64, t26538: f64, t27199: f64, t4487: f64, t7070: f64, t7403: f64, t7420: f64, t2718: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28399, t28400, t28404, t28405, t28411, t28417, t28418) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1780(t233, t28340, t1957, t2061, t231, t4423, t7076, t25317, t8006, t886, t4533, t7071);
        let t28424 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1781(t27213, t7407, t1956, t26508, t26521, t26522, t26529, t26534, t26536, t26538, t27199, t28400, t28405, t28411, t28418, t4487, t7070, t7403, t7420);
        let t28425 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1782(t2061, t2718);
    (t28399, t28400, t28404, t28405, t28411, t28417, t28418, t28424, t28425)
}
