//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1717;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta463<F: Float>(t26333: F, t545: F, t2028: F, t225: F, t26079: F, t26255: F, t4003: F, t1444: F, t7296: F, t7506: F, t2097: F, t4131: F) -> (F, F, F, F, F, F) {
        let (t26334, t26335, t26338, t26343, t26347, t26351) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1717::<F>(t26333, t545, t2028, t225, t26079, t26255, t4003, t1444, t7296, t7506, t2097, t4131);
    (t26334, t26335, t26338, t26343, t26347, t26351)
}
