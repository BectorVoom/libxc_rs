//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta564 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1963;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1964;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta564<F: Float>(t30735: F, t7637: F, t2142: F, t6573: F, t1769: F, t8190: F, t1774: F, t6563: F, t1828: F, t8201: F, t7652: F, t1794: F, t8208: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t30736, t30739, t30740, t30743, t30744, t30747) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1963::<F>(t30735, t7637, t2142, t6573, t1769, t8190, t1774);
        let (t30748, t30751, t30752, t30757, t30758, t30763) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1964::<F>(t30747, t7637, t2142, t6563, t1828, t8201, t7652, t1794, t8208);
    (t30736, t30739, t30740, t30743, t30744, t30747, t30748, t30751, t30752, t30757, t30758, t30763)
}
