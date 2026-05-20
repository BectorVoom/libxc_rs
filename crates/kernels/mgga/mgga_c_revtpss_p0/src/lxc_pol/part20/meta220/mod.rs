//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1007;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta220<F: Float>(t760: F, t9419: F, t2516: F, t2523: F, t9387: F, t2496: F, t189: F, t606: F, t2258: F, t4401: F, t9372: F, t37: F, t716: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10592, t10594, t10596, t10598, t10599, t10600, t10602, t10604, t10605) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1007::<F>(t760, t9419, t2516, t2523, t9387, t2496, t189, t606, t2258, t4401, t9372, t37, t716);
    (t10592, t10594, t10596, t10598, t10599, t10600, t10602, t10604, t10605)
}
