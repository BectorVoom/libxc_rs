//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1719;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta379<F: Float>(t4930: F, t994: F, t1678: F, t3046: F, t3057: F, t379: F, t1078: F, t1651: F, t342: F, t1071: F, t1647: F, t378: F, t4743: F) -> (F, F, F, F, F, F, F) {
        let (t16302, t16305, t16312, t16313, t16333, t16340, t16362) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1719::<F>(t4930, t994, t1678, t3046, t3057, t379, t1078, t1651, t342, t1071, t1647, t378, t4743);
    (t16302, t16305, t16312, t16313, t16333, t16340, t16362)
}
