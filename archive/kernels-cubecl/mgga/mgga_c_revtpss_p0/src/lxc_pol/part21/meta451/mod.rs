//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1978;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta451<F: Float>(t13312: F, t190: F, t706: F, t4391: F, t705: F, t707: F, t189: F, t4186: F, t606: F, t4401: F, t10579: F, t2411: F, t4537: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14383, t14385, t14386, t14388, t14389, t14390, t14392, t14396, t14397) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1978::<F>(t13312, t190, t706, t4391, t705, t707, t189, t4186, t606, t4401, t10579, t2411, t4537);
    (t14383, t14385, t14386, t14388, t14389, t14390, t14392, t14396, t14397)
}
