//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1901;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta462<F: Float>(t5825: F, t999: F, t4872: F, t1042: F, t1651: F, t905: F, t4873: F, t3092: F, t357: F, t4866: F, t4893: F, t3117: F) -> (F, F, F, F, F, F, F, F) {
        let (t19701, t19702, t19705, t19706, t19707, t19716, t19717, t19718) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1901::<F>(t5825, t999, t4872, t1042, t1651, t905, t4873, t3092, t357, t4866, t4893, t3117);
    (t19701, t19702, t19705, t19706, t19707, t19716, t19717, t19718)
}
