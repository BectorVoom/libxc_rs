//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1866;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta445<F: Float>(t11387: F, t6109: F, t934: F, t11385: F, t6158: F, t953: F, t1622: F, t4669: F, t6177: F, t6174: F, t2970: F, t6173: F) -> (F, F, F, F, F, F, F, F) {
        let (t19255, t19256, t19258, t19263, t19266, t19269, t19272, t19275) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1866::<F>(t11387, t6109, t934, t11385, t6158, t953, t1622, t4669, t6177, t6174, t2970, t6173);
    (t19255, t19256, t19258, t19263, t19266, t19269, t19272, t19275)
}
