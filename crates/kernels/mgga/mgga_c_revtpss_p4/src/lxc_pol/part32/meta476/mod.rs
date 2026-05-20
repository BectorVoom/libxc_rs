//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1710;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1711;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1712;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta476<F: Float>(t116: F, t7373: F, t1518: F, t648: F, t4292: F, t94: F, t1353: F, t1907: F, t30: F, t892: F, t4433: F, t18875: F, t25207: F, t1544: F, t605: F, t4343: F, t1032: F, t1568: F, t1955: F, t867: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26733, t27123) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1710::<F>(t116, t7373, t1518, t648);
        let (t27126, t27153, t27159, t27160, t27166, t27169, t27173) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1711::<F>(t4292, t94, t1353, t1907, t30, t892, t4433, t18875, t25207, t1544, t605, t4343);
        let (t27198, t27199) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1712::<F>(t1032, t1568, t1955);
        let t27212 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1713::<F>(t27198, t867);
    (t26733, t27123, t27126, t27153, t27159, t27160, t27166, t27169, t27173, t27198, t27199, t27212)
}
