//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1220;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta302<F: Float>(t10544: F, t2784: F, t892: F, t2841: F, t888: F, t2840: F, t287: F, t275: F, t10294: F, t891: F, t2843: F, t290: F) -> (F, F, F, F, F, F, F, F) {
        let (t10636, t10650, t10655, t10661, t10675, t10676, t10702, t10704) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1220::<F>(t10544, t2784, t892, t2841, t888, t2840, t287, t275, t10294, t891, t2843, t290);
    (t10636, t10650, t10655, t10661, t10675, t10676, t10702, t10704)
}
