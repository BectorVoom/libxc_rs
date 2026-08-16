//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta634 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2404;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2405;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta634<F: Float>(t11003: F, t9303: F, t10978: F, t689: F, t779: F, t10981: F, t22: F, t868: F, t886: F, t10910: F, t212: F, t780: F, t10988: F, t2435: F, t2445: F, t9292: F, t11025: F, t588: F, t10991: F, t39497: F, t787: F, t788: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t40970, t40973, t40978, t40982) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2404::<F>(t11003, t9303, t10978, t689, t779, t10981, t22, t868, t886, t10910, t212, t780);
        let (t40986, t40988, t40994, t40998, t40999, t41003) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2405::<F>(t10988, t2435, t2445, t9292, t11025, t10981, t588, t780, t10991, t39497, t787, t788);
    (t40970, t40973, t40978, t40982, t40986, t40988, t40994, t40998, t40999, t41003)
}
