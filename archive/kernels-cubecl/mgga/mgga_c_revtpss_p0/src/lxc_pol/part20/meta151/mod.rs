//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta151 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk825;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk826;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk827;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta151<F: Float>(t1399: F, t221: F, t4019: F, t4018: F, t1317: F, t1331: F, t1333: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t3852: F, t3854: F, t3871: F, t3873: F, t1330: F, t749: F, t512: F, t1320: F, t1340: F, t2516: F, t2496: F, t177: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4021, t4022, t4025, t4027, t4028) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk825::<F>(t1399, t221, t4019, t4018, t1317, t1331, t1333, t2522, t2562, t2569, t2579, t2587, t3852, t3854, t3871, t3873);
        let t4029 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk826::<F>(t1330, t749);
        let (t4031, t4033, t4035, t4037, t4038) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk827::<F>(t4029, t512, t1320, t1331, t1340, t2516, t2496, t1330, t177);
    (t4021, t4022, t4025, t4027, t4028, t4029, t4031, t4033, t4035, t4037, t4038)
}
