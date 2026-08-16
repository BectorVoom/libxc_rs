//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta773 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2859;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2860;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta773<F: Float>(t3367: F, t3603: F, t2251: F, t12839: F, t2258: F, t3555: F, t3766: F, t5330: F, t1209: F, t13147: F, t17708: F, t12854: F, t17350: F, t12808: F, t12865: F, t12909: F, t13037: F, t472: F, t482: F, t675: F, t828: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t44458, t44459, t44466, t44484, t44500, t44510) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2859::<F>(t3367, t3603, t2251, t12839, t2258, t3555, t3766, t5330, t1209, t13147, t17708, t12854, t17350);
        let (t44517, t44521, t44531, t44535, t44545, t44546) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2860::<F>(t12808, t17350, t12865, t12909, t13037, t472, t3603, t482, t675, t828);
    (t44458, t44459, t44466, t44484, t44500, t44510, t44517, t44521, t44531, t44535, t44545, t44546)
}
