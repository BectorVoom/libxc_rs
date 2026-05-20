//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta291 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1074;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta291<F: Float>(t12256: F, t5819: F, t12268: F, t3367: F, t5825: F, t12327: F, t6442: F, t12331: F, t300: F, t6513: F, t12485: F, t6518: F) -> (F, F, F, F, F, F, F) {
        let (t20292, t20297, t20317, t20356, t20365, t20400, t20472) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1074::<F>(t12256, t5819, t12268, t3367, t5825, t12327, t6442, t12331, t300, t6513, t12485, t6518);
    (t20292, t20297, t20317, t20356, t20365, t20400, t20472)
}
