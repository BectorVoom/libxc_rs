//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta241 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk918;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta241<F: Float>(t225: F, t494: F, t5412: F, t1811: F, t460: F, t1214: F, t1828: F, t1277: F, t1294: F, t3737: F, t1284: F, t1770: F) -> (F, F, F, F, F) {
        let (t5414, t5417, t5423, t5429, t5436) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk918::<F>(t225, t494, t5412, t1811, t460, t1214, t1828, t1277, t1294, t3737, t1284, t1770);
    (t5414, t5417, t5423, t5429, t5436)
}
