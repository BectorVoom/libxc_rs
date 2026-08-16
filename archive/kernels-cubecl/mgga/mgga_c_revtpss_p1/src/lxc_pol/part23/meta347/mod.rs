//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta347 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1650;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1651;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta347<F: Float>(t4424: F, t72: F, t686: F, t2798: F, t136: F, t1559: F, t2457: F, t10535: F, t10069: F, t4496: F, t1568: F, t836: F, t231: F, t2783: F, t2782: F, t10867: F, t225: F, t213: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14519, t14520, t14522, t14523, t14524, t14525, t14533, t14535) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1650::<F>(t4424, t72, t686, t2798, t136, t1559, t2457, t10535, t10069, t4496, t1568, t836);
        let (t14537, t14539, t14545, t14546) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1651::<F>(t14535, t231, t2783, t2782, t10867, t225, t213);
    (t14519, t14520, t14522, t14523, t14524, t14525, t14533, t14537, t14539, t14545, t14546)
}
