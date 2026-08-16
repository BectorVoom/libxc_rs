//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1261;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta371<F: Float>(t24493: F, t3523: F, t1196: F, t1179: F, t1188: F, t24407: F, t1832: F, t6752: F, t1828: F, t3737: F, t6744: F, t1774: F) -> (F, F, F, F, F, F, F) {
        let (t24494, t24496, t24498, t24500, t24501, t24509, t24514) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1261::<F>(t24493, t3523, t1196, t1179, t1188, t24407, t1832, t6752, t1828, t3737, t6744, t1774);
    (t24494, t24496, t24498, t24500, t24501, t24509, t24514)
}
