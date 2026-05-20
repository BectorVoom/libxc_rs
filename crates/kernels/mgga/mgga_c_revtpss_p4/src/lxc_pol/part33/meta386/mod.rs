//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta386<F: Float>(t342: F, t4930: F, t1071: F, t1647: F, t378: F, t4743: F, t1678: F, t989: F, t15654: F, t1086: F, t359: F, t3286: F, t4746: F) -> (F, F, F, F, F, F, F, F) {
        let (t16333, t16340, t16362, t16371, t16374, t16381, t16449, t16502) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1433::<F>(t342, t4930, t1071, t1647, t378, t4743, t1678, t989, t15654, t1086, t359, t3286, t4746);
    (t16333, t16340, t16362, t16371, t16374, t16381, t16449, t16502)
}
