//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta374 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1339;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta374(t3936: f64, t4004: f64, t5704: f64, t3924: f64, t2482: f64, t4000: f64, t814: f64, t136: f64, t550: f64, t220: f64, t124: f64, t1882: f64, t5675: f64, t5673: f64, t5674: f64, t5609: f64, t9794: f64, t9793: f64, t13817: f64, t13821: f64, t13826: f64, t13832: f64, t1410: f64, t3934: f64, t5671: f64, t9739: f64, t9742: f64, t9745: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13834, t13841, t13845, t13846, t13847, t13848) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1339(t3936, t4004, t5704, t3924, t2482, t4000, t814, t136, t550, t220, t124, t1882);
        let (t13850, t13854, t13857, t13860) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1340(t13847, t13848, t5675, t13845, t3924, t5673, t5674, t5609, t9794, t9793, t13817, t13821, t13826, t13832, t13834, t13841, t1410, t3934, t5671, t9739, t9742, t9745);
    (t13834, t13841, t13846, t13847, t13848, t13850, t13854, t13857, t13860)
}
