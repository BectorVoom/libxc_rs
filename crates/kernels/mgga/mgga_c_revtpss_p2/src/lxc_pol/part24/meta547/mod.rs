//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta547 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1620;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta547<F: Float>(t14330: F, t18305: F, t5819: F, t190: F, t2611: F, t87107: F, t23121: F, t50089: F, t50084: F, t50092: F, t50094: F, t40088: F, t40099: F, t40103: F, t40115: F, t40131: F, t40137: F) -> (F, F, F, F, F, F, F) {
        let (t87655, t87658, t87660, t87661, t87662, t87663, t87664) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1620::<F>(t14330, t18305, t5819, t190, t2611, t87107, t23121, t50089, t50084, t50092, t50094, t40088, t40099, t40103, t40115, t40131, t40137);
    (t87655, t87658, t87660, t87661, t87662, t87663, t87664)
}
