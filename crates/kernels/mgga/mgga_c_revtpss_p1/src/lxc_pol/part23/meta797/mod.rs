//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta797 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2620;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2621;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta797<F: Float>(t10811: F, t18639: F, t10905: F, t18507: F, t10777: F, t10779: F, t2749: F, t61715: F, t18651: F, t14923: F, t18456: F, t14671: F, t14686: F, t14931: F, t18632: F, t4424: F, t61956: F, t837: F, t18477: F, t50769: F, t51133: F, t18348: F, t2710: F, t2713: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t62162, t62168, t62176, t62178, t62188, t62216) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2620::<F>(t10811, t18639, t10905, t18507, t10777, t10779, t2749, t61715, t18651, t14923, t18456, t14671, t14686, t14931, t18632);
        let (t62236, t62241, t62246, t62251) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2621::<F>(t10777, t14671, t14686, t4424, t61956, t837, t18477, t50769, t51133, t18348, t2710, t2713);
    (t62162, t62168, t62176, t62178, t62188, t62216, t62236, t62241, t62246, t62251)
}
