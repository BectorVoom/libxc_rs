//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1337;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta311<F: Float>(t10629: F, t315: F, t2885: F, t919: F, t2884: F, t307: F, t302: F, t10294: F, t10544: F, t922: F, t2887: F, t310: F) -> (F, F, F, F, F, F, F) {
        let (t10756, t10765, t10771, t10784, t10785, t10811, t10813) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1337::<F>(t10629, t315, t2885, t919, t2884, t307, t302, t10294, t10544, t922, t2887, t310);
    (t10756, t10765, t10771, t10784, t10785, t10811, t10813)
}
