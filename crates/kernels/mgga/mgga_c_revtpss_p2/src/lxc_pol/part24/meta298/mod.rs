//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1083;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta298<F: Float>(t6622: F, t73: F, t17934: F, t5330: F, t5327: F, t5362: F, t1803: F, t5326: F, t5323: F, t12772: F, t6639: F, t3625: F) -> (F, F, F, F, F, F, F) {
        let (t21040, t21049, t21053, t21063, t21088, t21090, t21091) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1083::<F>(t6622, t73, t17934, t5330, t5327, t5362, t1803, t5326, t5323, t12772, t6639, t3625);
    (t21040, t21049, t21053, t21063, t21088, t21090, t21091)
}
