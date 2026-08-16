//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta224 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk977;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk978;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta224<F: Float>(t12428: F, t426: F, t12295: F, t12351: F, t1159: F, t3475: F, t3478: F, t434: F, t3519: F, t444: F, t439: F, t1178: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12429, t12459, t12460, t12469, t12470, t12472, t12485) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk977::<F>(t12428, t426, t12295, t12351, t1159, t3475, t3478, t434, t3519, t444);
        let (t12486, t12542, t12543, t12552) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk978::<F>(t12485, t439, t12295, t12351, t1178, t3519);
    (t12429, t12459, t12460, t12469, t12470, t12472, t12485, t12486, t12542, t12543, t12552)
}
