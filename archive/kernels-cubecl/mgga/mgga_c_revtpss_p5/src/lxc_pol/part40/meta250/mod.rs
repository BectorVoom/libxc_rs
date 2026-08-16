//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta250 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk937;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk938;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta250<F: Float>(t3978: F, t5622: F, t1885: F, t3930: F, t1353: F, t1868: F, t4012: F, t828: F, t3826: F, t187: F, t5566: F, t1856: F, t72: F, t757: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t5546: F, t5548: F, t5568: F, t5570: F, t5573: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t5623, t5625, t5627, t5629, t5632, t5634, t5635) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk937::<F>(t3978, t5622, t1885, t3930, t1353, t1868, t4012, t828, t3826, t187, t5566, t1856, t72);
        let (t5637, t5638) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk938::<F>(t5635, t757, t2522, t2562, t2569, t2579, t2587, t5546, t5548, t5568, t5570, t5573, t5632, t5634);
    (t5623, t5625, t5627, t5629, t5632, t5634, t5635, t5637, t5638)
}
