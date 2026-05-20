//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta678 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2416;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2417;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta678<F: Float>(t1214: F, t13045: F, t12854: F, t17350: F, t12808: F, t12865: F, t12909: F, t13051: F, t44173: F, t13037: F, t472: F, t3603: F, t482: F, t675: F, t828: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t44502, t44510, t44517, t44521, t44526, t44531, t44535) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2416::<F>(t1214, t13045, t12854, t17350, t12808, t12865, t12909, t13051, t44173, t13037, t472, t3603);
        let (t44545, t44546) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2417::<F>(t482, t675, t828);
    (t44502, t44510, t44517, t44521, t44526, t44531, t44535, t44545, t44546)
}
