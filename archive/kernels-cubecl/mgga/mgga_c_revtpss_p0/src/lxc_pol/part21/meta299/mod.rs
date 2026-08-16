//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1549;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta299<F: Float>(t10578: F, t2630: F, t2629: F, t9866: F, t9575: F, t9572: F, t177: F, t2390: F, t762: F, t10575: F, t10577: F, t9514: F, t9517: F, t9521: F, t9524: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10579, t10580, t10582, t10584, t10586, t10587, t10588, t10589, t10590) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1549::<F>(t10578, t2630, t2629, t9866, t9575, t9572, t177, t2390, t762, t10575, t10577, t9514, t9517, t9521, t9524);
    (t10579, t10580, t10582, t10584, t10586, t10587, t10588, t10589, t10590)
}
