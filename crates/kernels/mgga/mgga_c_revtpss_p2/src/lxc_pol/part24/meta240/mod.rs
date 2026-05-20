//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta240 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1001;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta240<F: Float>(t14219: F, t2457: F, t10139: F, t1892: F, t4086: F, t786: F, t2470: F, t5740: F, t4101: F, t1432: F, t5763: F, t3920: F, t5603: F) -> (F, F, F, F, F, F, F, F) {
        let (t14220, t14221, t14238, t14239, t14242, t14243, t14252, t14280) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1001::<F>(t14219, t2457, t10139, t1892, t4086, t786, t2470, t5740, t4101, t1432, t5763, t3920, t5603);
    (t14220, t14221, t14238, t14239, t14242, t14243, t14252, t14280)
}
