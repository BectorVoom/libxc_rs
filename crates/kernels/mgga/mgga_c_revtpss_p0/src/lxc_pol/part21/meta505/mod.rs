//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta505 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2124;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta505<F: Float>(t4186: F, t999: F, t4872: F, t1042: F, t4866: F, t73: F) -> (F, F, F, F) {
        let (t15950, t15951, t15952, t15957) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2124::<F>(t4186, t999, t4872, t1042, t4866, t73);
    (t15950, t15951, t15952, t15957)
}
