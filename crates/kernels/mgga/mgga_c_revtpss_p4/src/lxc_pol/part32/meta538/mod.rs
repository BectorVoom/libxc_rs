//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta538 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta538<F: Float>(t25386: F, t95536: F, t26518: F, t9285: F, t25299: F, t2061: F, t22: F, t25402: F, t93140: F, t25310: F, t26506: F, t2439: F, t7398: F, t780: F, t785: F) -> (F, F, F, F, F, F, F) {
        let (t95537, t95540, t95542, t95546, t95548, t95551, t95562) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1849::<F>(t25386, t95536, t26518, t9285, t25299, t2061, t22, t25402, t93140, t25310, t26506, t2439, t7398, t780, t785);
    (t95537, t95540, t95542, t95546, t95548, t95551, t95562)
}
