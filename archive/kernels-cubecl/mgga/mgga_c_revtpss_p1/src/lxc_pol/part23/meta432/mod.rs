//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1832;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta432<F: Float>(t1587: F, t2: F, t580: F, t11506: F, t6189: F, t11509: F, t972: F, t981: F, t11144: F, t5819: F, t606: F, t11142: F, t128: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t18892, t18898, t18899, t18900, t18902, t18903, t18904, t18905) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1832::<F>(t1587, t2, t580, t11506, t6189, t11509, t972, t981, t11144, t5819, t606, t11142);
        let t18906 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1833::<F>(t128, t18905);
    (t18892, t18898, t18899, t18900, t18902, t18903, t18904, t18905, t18906)
}
