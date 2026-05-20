//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta423 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1371;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1372;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta423<F: Float>(t12808: F, t17350: F, t12865: F, t12909: F, t13037: F, t472: F, t44372: F, t44373: F, t474: F, t3603: F, t42871: F, t482: F, t675: F, t828: F, t3566: F, t3766: F, t5330: F, t1209: F, t13141: F, t17708: F, t371: F, t481: F, t9291: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44517, t44521, t44531, t44534, t44535, t44536, t44545) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1371::<F>(t12808, t17350, t12865, t12909, t13037, t472, t44372, t44373, t474, t3603, t42871, t482, t675);
        let (t44546, t44551, t44578, t44607) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1372::<F>(t44545, t828, t3566, t3766, t5330, t1209, t13141, t17708, t371, t481, t482, t9291);
    (t44517, t44521, t44531, t44534, t44535, t44536, t44546, t44551, t44578, t44607)
}
