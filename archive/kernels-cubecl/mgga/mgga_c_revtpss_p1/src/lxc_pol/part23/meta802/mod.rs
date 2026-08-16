//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta802 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2630;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta802<F: Float>(t23160: F, t836: F, t10529: F, t2782: F, t14520: F, t14606: F, t6016: F, t860: F, t231: F, t2783: F, t18657: F, t686: F, t72: F, t874: F, t1559: F, t4423: F, t2797: F, t14586: F, t18725: F, t2470: F, t2798: F, t10542: F, t18730: F, t61749: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t62606, t62609, t62612, t62615, t62619) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2630::<F>(t23160, t836, t10529, t2782, t14520, t14606, t6016, t860, t231, t2783, t18657, t686, t72, t874);
        let (t62626, t62630, t62633, t62635, t62637) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2631::<F>(t1559, t4423, t2782, t2797, t14586, t10529, t18725, t2470, t2798, t10542, t18730, t231, t61749);
    (t62606, t62609, t62612, t62615, t62619, t62626, t62630, t62633, t62635, t62637)
}
