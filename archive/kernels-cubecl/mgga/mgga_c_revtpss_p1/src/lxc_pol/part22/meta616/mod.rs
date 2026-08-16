//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2523;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta616<F: Float>(t19680: F, t4806: F, t1042: F, t5819: F, t999: F, t1032: F, t6235: F, t1040: F, t5825: F, t4872: F, t1651: F, t905: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19687, t19688, t19691, t19692, t19693, t19696, t19697, t19700, t19701, t19702, t19705) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2523::<F>(t19680, t4806, t1042, t5819, t999, t1032, t6235, t1040, t5825, t4872, t1651, t905);
    (t19687, t19688, t19691, t19692, t19693, t19696, t19697, t19700, t19701, t19702, t19705)
}
