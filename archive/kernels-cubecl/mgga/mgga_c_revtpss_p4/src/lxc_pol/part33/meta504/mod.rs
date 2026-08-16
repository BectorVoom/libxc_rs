//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta504 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1823;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta504<F: Float>(t13426: F, t1937: F, t18227: F, t4248: F, t6993: F, t7003: F, t1518: F, t648: F) -> (F, F, F, F, F) {
        let (t27116, t27118, t27120, t27122, t27123) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1823::<F>(t13426, t1937, t18227, t4248, t6993, t7003, t1518, t648);
    (t27116, t27118, t27120, t27122, t27123)
}
