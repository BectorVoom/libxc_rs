//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta817 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2927;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2928;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta817<F: Float>(t1904: F, t2439: F, t9640: F, t5718: F, t9292: F, t14274: F, t2435: F, t4078: F, t5599: F, t689: F, t13734: F, t1445: F, t10175: F, t14090: F, t14100: F, t9671: F, t1357: F, t14269: F, t1358: F, t14066: F, t212: F, t13746: F, t686: F, t72: F, t9680: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t47800, t47802, t47805, t47808, t47811) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2927::<F>(t1904, t2439, t9640, t5718, t9292, t14274, t2435, t4078, t5599, t689, t13734, t1445);
        let (t47813, t47816, t47819, t47825, t47832) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2928::<F>(t10175, t14090, t14100, t9671, t1357, t14269, t689, t1358, t14066, t212, t13746, t686, t72, t9680);
    (t47800, t47802, t47805, t47808, t47811, t47813, t47816, t47819, t47825, t47832)
}
