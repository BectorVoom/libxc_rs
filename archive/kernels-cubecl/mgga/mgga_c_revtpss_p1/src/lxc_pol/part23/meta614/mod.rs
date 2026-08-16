//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta614 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2287;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta614<F: Float>(t5192: F, t6552: F, t1188: F, t24375: F, t3520: F, t1196: F, t1765: F, t20400: F, t5197: F, t6535: F, t6556: F, t12485: F) -> (F, F, F, F, F, F, F, F) {
        let (t24478, t24480, t24482, t24484, t24488, t24490, t24492, t24493) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2287::<F>(t5192, t6552, t1188, t24375, t3520, t1196, t1765, t20400, t5197, t6535, t6556, t12485);
    (t24478, t24480, t24482, t24484, t24488, t24490, t24492, t24493)
}
