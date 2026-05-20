//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta594 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2236;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta594<F: Float>(t23842: F, t4806: F, t1042: F, t23633: F, t4801: F, t1651: F, t5825: F, t4872: F, t1592: F, t19649: F, t1015: F, t22671: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23843, t23844, t23847, t23848, t23851, t23852, t23857, t23858, t23859, t23862, t23863, t23868) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2236::<F>(t23842, t4806, t1042, t23633, t4801, t1651, t5825, t4872, t1592, t19649, t1015, t22671);
    (t23843, t23844, t23847, t23848, t23851, t23852, t23857, t23858, t23859, t23862, t23863, t23868)
}
