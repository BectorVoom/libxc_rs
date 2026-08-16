//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta82 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk480;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk481;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk482;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk483;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta82<F: Float>(t1598: F, t1612: F, t1614: F, t1622: F, t1627: F, t1634: F, t300: F, t311: F, t946: F, t965: F, t1633: F, t964: F, t973: F, t981: F, t1594: F, t986: F, t341: F, t997: F, t996: F, t1015: F, t1469: F, t1012: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1638, t1640, t1642) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk480::<F>(t1598, t1612, t1614, t1622, t1627, t1634, t300, t311, t946, t965, t1633, t964, t973);
        let (t1644, t1646, t1647) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk481::<F>(t1642, t981, t1594, t986, t341);
        let t1651 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk482::<F>(t1594, t997);
        let (t1652, t1655, t1656, t1659) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk483::<F>(t1651, t996, t1015, t1469, t1012, t1647, t225);
    (t1638, t1640, t1642, t1644, t1646, t1647, t1651, t1652, t1655, t1656, t1659)
}
