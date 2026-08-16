//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta363 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1679;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1680;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta363<F: Float>(t15127: F, t2852: F, t4186: F, t2857: F, t4625: F, t698: F, t4622: F, t1593: F, t2435: F, t4584: F, t689: F) -> (F, F, F, F, F, F, F, F) {
        let (t15128, t15129, t15144, t15168, t15169, t15170, t15189) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1679::<F>(t15127, t2852, t4186, t2857, t4625, t698, t4622, t1593, t2435);
        let t15191 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1680::<F>(t4584, t689);
    (t15128, t15129, t15144, t15168, t15169, t15170, t15189, t15191)
}
