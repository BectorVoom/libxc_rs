//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1730;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1731;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta485<F: Float>(t1873: F, t26004: F, t5690: F, t7252: F, t1398: F, t1903: F, t543: F, t1955: F, t5710: F, t1513: F, t25823: F, t665: F, t25826: F, t4287: F, t6998: F, t4237: F, t76: F, t13269: F, t38: F, t1497: F, t640: F, t77: F, t4241: F, t84: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27955, t27957, t27972, t28008, t28034, t28036) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1730::<F>(t1873, t26004, t5690, t7252, t1398, t1903, t543, t1955, t5710, t1513, t25823, t665);
        let (t28037, t28039, t28089, t28093, t28105, t28108) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1731::<F>(t25826, t28036, t4287, t6998, t4237, t76, t13269, t38, t1497, t640, t77, t4241, t84);
    (t27955, t27957, t27972, t28008, t28034, t28036, t28037, t28039, t28089, t28093, t28105, t28108)
}
