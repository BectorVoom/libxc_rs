//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta327 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1116;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1117;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta327<F: Float>(t14519: F, t686: F, t2798: F, t136: F, t1559: F, t2457: F, t10535: F, t10069: F, t4496: F, t1568: F, t836: F, t231: F, t2783: F, t2782: F, t10867: F, t225: F, t213: F, t2777: F, t4518: F, t2439: F, t2470: F, t4499: F) -> (F, F, F, F, F, F, F, F) {
        let (t14522, t14525, t14533, t14537) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1116::<F>(t14519, t686, t2798, t136, t1559, t2457, t10535, t10069, t4496, t1568, t836, t231, t2783);
        let (t14539, t14546, t14558, t14564, t14567) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1117::<F>(t14537, t2782, t10867, t225, t213, t2777, t4518, t2439, t2470, t4499, t2798, t1568, t2783);
    (t14522, t14525, t14533, t14539, t14546, t14558, t14564, t14567)
}
