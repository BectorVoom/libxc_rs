//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta592 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2063;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta592<F: Float>(t1399: F, t2434: F, t25880: F, t25899: F, t3924: F, t676: F, t2022: F, t9646: F, t9648: F, t25875: F, t94394: F, t94398: F) -> (F, F, F, F, F, F, F) {
        let (t94634, t94635, t94640, t94641, t94648, t94649, t94650) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2063::<F>(t1399, t2434, t25880, t25899, t3924, t676, t2022, t9646, t9648, t25875, t94394, t94398);
    (t94634, t94635, t94640, t94641, t94648, t94649, t94650)
}
