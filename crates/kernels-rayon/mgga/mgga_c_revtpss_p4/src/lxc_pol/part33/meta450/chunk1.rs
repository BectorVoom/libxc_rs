//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1638/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1638(t20849: f64, t225: f64, t480: f64, t1238: f64, t17296: f64, t17298: f64, t17301: f64, t17304: f64, t17337: f64, t17609: f64, t1797: f64, t20838: f64, t20843: f64, t20847: f64, t5274: f64, t5287: f64, t5293: f64, t5331: f64) -> (f64, f64) {
    let t20850 = t20849 * t225;
    let t20851 = t20850 * t480;
    let t20855 = -0.22866142996303859718e-2_f64 * t5293 * t5287 + 0.42874018118069736972e-3_f64 * t17609 * t1797 + 0.42874018118069736972e-3_f64 * t5274 * t5287 - 0.42874018118069736972e-3_f64 * t5331 * t20838 - 0.14291339372689912324e-3_f64 * t20843 + 0.28582678745379824648e-3_f64 * t20847 - 0.21437009059034868486e-3_f64 * t20851 * t1238 - t17296 + t17298 - t17301 + 0.95275595817932748827e-4_f64 * t17304 - t17337;
    (t20850, t20855)
}
