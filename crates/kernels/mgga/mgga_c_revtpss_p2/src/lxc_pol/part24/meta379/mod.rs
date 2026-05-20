//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1274;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta379<F: Float>(t1264: F, t24240: F, t247: F, t1794: F, t3603: F, t20800: F, t3720: F, t471: F, t6573: F, t1250: F, t17661: F, t6639: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t24726, t24729, t24730, t24731, t24734, t24735, t24736, t24739, t24740, t24741, t24744) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1274::<F>(t1264, t24240, t247, t1794, t3603, t20800, t3720, t471, t6573, t1250, t17661, t6639);
    (t24726, t24729, t24730, t24731, t24734, t24735, t24736, t24739, t24740, t24741, t24744)
}
