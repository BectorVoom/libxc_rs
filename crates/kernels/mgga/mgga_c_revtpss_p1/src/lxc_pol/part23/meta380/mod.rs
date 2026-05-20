//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta380 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1720;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1721;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta380<F: Float>(t1678: F, t989: F, t15654: F, t378: F, t1086: F, t4743: F, t1071: F, t3298: F, t342: F, t3302: F, t4893: F, t359: F, t4930: F, t3286: F, t4746: F) -> (F, F, F, F, F, F, F, F) {
        let (t16371, t16374, t16381, t16409, t16410, t16432, t16449) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1720::<F>(t1678, t989, t15654, t378, t1086, t4743, t1071, t3298, t342, t3302, t4893, t359, t4930);
        let t16502 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1721::<F>(t3286, t4746);
    (t16371, t16374, t16381, t16409, t16410, t16432, t16449, t16502)
}
