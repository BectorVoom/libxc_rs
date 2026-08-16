//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta253 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1036;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1037;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1038;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta253<F: Float>(t4012: F, t5627: F, t828: F, t3826: F, t187: F, t5566: F, t1856: F, t72: F, t757: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t5546: F, t5548: F, t5568: F, t5570: F, t5573: F, t4039: F, t4032: F, t4024: F, t3854: F, t3859: F, t3862: F, t3867: F, t3871: F, t3873: F, t4030: F, t4035: F, t4037: F, t4042: F, t225: F, t539: F, t73: F, t1412: F, t1868: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5629, t5632, t5634, t5635, t5637, t5638) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1036::<F>(t4012, t5627, t828, t3826, t187, t5566, t1856, t72, t757, t2522, t2562, t2569, t2579, t2587, t5546, t5548, t5568, t5570, t5573);
        let (t5639, t5640, t5641, t5642) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1037::<F>(t4039, t4032, t4024, t3854, t3859, t3862, t3867, t3871, t3873, t4030, t4035, t4037, t4042);
        let (t5644, t5650, t5651) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1038::<F>(t225, t5638, t5642, t539, t73, t1412, t1868);
    (t5629, t5632, t5634, t5635, t5637, t5639, t5640, t5641, t5644, t5650, t5651)
}
