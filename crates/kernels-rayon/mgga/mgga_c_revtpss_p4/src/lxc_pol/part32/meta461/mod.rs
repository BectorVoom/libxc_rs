//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1682;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1683;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1684;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta461(t239: f64, t25981: f64, t820: f64, t240: f64, t7262: f64, t3994: f64, t2661: f64, t2482: f64, t27: f64, t4021: f64, t25273: f64, t533: f64, t816: f64, t540: f64, t7021: f64, t1372: f64, t1389: f64, t7269: f64, t2736: f64, t2689: f64, t7256: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25983, t25986) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1682(t239, t25981, t820, t240, t7262);
        let (t25987, t25989, t25997) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1683(t25986, t3994, t2661, t2482, t27, t7262);
        let (t25998, t26002, t26004, t26006, t26009, t26010, t26012) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1684(t25997, t4021, t25273, t533, t816, t540, t7021, t1372, t1389, t7269, t2736, t2689, t7256);
    (t25983, t25986, t25987, t25989, t25997, t25998, t26002, t26004, t26006, t26009, t26010, t26012)
}
