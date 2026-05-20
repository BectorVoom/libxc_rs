//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta448 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1869;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta448<F: Float>(t15421: F, t4636: F, t6110: F, t934: F, t2924: F, t1610: F, t4631: F, t2874: F, t6145: F, t11299: F, t6142: F, t2926: F, t6141: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19317, t19318, t19320, t19321, t19323, t19324, t19326, t19327, t19329, t19330) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1869::<F>(t15421, t4636, t6110, t934, t2924, t1610, t4631, t2874, t6145, t11299, t6142, t2926, t6141);
    (t19317, t19318, t19320, t19321, t19323, t19324, t19326, t19327, t19329, t19330)
}
