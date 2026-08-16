//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1842;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1843;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta520(t2482: f64, t7262: f64, t814: f64, t9821: f64, t820: f64, t844: f64, t3940: f64, t596: f64, t7269: f64, t3981: f64, t25986: f64, t2661: f64, t9930: f64, t25981: f64, t843: f64, t4006: f64, t2681: f64, t1401: f64, t25997: f64, t9905: f64, t533: f64, t816: f64, t92993: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94423, t94424, t94429, t94430, t94443, t94444, t94449) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1842(t2482, t7262, t814, t9821, t820, t844, t3940, t596, t7269, t3981, t25986, t2661, t9930);
        let (t94456, t94459, t94460, t94468, t94471) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1843(t25981, t820, t843, t4006, t2681, t7262, t1401, t25997, t9905, t533, t816, t92993);
    (t94423, t94424, t94429, t94430, t94443, t94444, t94449, t94456, t94459, t94460, t94468, t94471)
}
