//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta845 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2724;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2725;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta845<F: Float>(t3666: F, t6594: F, t17283: F, t5362: F, t1222: F, t140: F, t21209: F, t21213: F, t3685: F, t12865: F, t5436: F, t3671: F, t371: F, t6609: F, t676: F, t1235: F, t127: F, t21083: F, t12967: F, t20846: F, t17708: F, t59550: F, t12916: F, t21299: F, t3718: F, t20842: F, t3667: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t70469, t70476, t70491, t70493, t70496, t70511) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2724::<F>(t3666, t6594, t17283, t5362, t1222, t140, t21209, t21213, t3685, t12865, t5436, t3671, t371, t6609, t676);
        let (t70521, t70523, t70530, t70542, t70581) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2725::<F>(t1235, t127, t21083, t371, t12967, t20846, t17708, t59550, t12916, t21299, t3718, t20842, t3667);
    (t70469, t70476, t70491, t70493, t70496, t70511, t70521, t70523, t70530, t70542, t70581)
}
