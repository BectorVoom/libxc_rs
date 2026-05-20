//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta459 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1430;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta459<F: Float>(t16543: F, t3057: F, t12077: F, t1647: F, t19602: F, t994: F, t19607: F, t12166: F, t4746: F, t4980: F, t1716: F, t9292: F) -> (F, F, F, F, F, F, F) {
        let (t55887, t55899, t55988, t55991, t56017, t56049, t56236) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1430::<F>(t16543, t3057, t12077, t1647, t19602, t994, t19607, t12166, t4746, t4980, t1716, t9292);
    (t55887, t55899, t55988, t55991, t56017, t56049, t56236)
}
