//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta241 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1002;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1003;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta241<F: Float>(t2435: F, t5718: F, t1893: F, t2453: F, t3908: F, t1904: F, t3895: F, t2439: F, t1532: F, t2609: F, t2626: F, t4398: F, t10439: F, t162: F, t2516: F, t2496: F, t2619: F, t4302: F, t123: F, t1534: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14290, t14293, t14294, t14296, t14297, t14312, t14328) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1002::<F>(t2435, t5718, t1893, t2453, t3908, t1904, t3895, t2439, t1532, t2609, t2626, t4398);
        let (t14330, t14334, t14336, t14339, t14362) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1003::<F>(t10439, t162, t2516, t4398, t2496, t2619, t4302, t123, t1534);
    (t14290, t14293, t14294, t14296, t14297, t14312, t14328, t14330, t14334, t14336, t14339, t14362)
}
