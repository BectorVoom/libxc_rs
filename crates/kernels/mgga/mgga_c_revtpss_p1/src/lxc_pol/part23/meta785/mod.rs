//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta785 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2595;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2596;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta785<F: Float>(t18550: F, t72: F, t757: F, t18299: F, t750: F, t18298: F, t705: F, t18281: F, t706: F, t18838: F, t892: F, t2609: F, t2611: F, t5819: F, t18544: F, t2398: F, t14440: F, t4311: F, t14386: F, t4305: F, t177: F, t762: F, t123: F, t2630: F, t5941: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t61093, t61114, t61122, t61130, t61139, t61165) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2595::<F>(t18550, t72, t757, t18299, t750, t18298, t705, t18281, t706, t18838, t892, t2609, t2611, t5819);
        let (t61178, t61180, t61201, t61239, t61247) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2596::<F>(t18544, t2398, t14440, t4311, t14386, t4305, t177, t18550, t762, t123, t2630, t5941);
    (t61093, t61114, t61122, t61130, t61139, t61165, t61178, t61180, t61201, t61239, t61247)
}
