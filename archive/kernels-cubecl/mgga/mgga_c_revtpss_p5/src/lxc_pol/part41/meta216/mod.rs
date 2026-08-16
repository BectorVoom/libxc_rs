//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta216 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk841;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta216<F: Float>(t4076: F, t5727: F, t1882: F, t555: F, t4086: F, t543: F, t2782: F, t1883: F, t72: F, t686: F, t4101: F, t225: F, t3999: F) -> (F, F, F, F, F, F, F, F) {
        let (t5728, t5735, t5737, t5738, t5740, t5741, t5742, t5744) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk841::<F>(t4076, t5727, t1882, t555, t4086, t543, t2782, t1883, t72, t686, t4101, t225, t3999);
    (t5728, t5735, t5737, t5738, t5740, t5741, t5742, t5744)
}
