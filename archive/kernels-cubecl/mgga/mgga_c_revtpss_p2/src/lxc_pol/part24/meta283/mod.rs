//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta283 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1059;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1060;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1061;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta283<F: Float>(t11465: F, t6189: F, t3336: F, t6396: F, t6184: F, t964: F, t6152: F, t945: F, t11387: F, t6109: F, t2970: F, t6173: F, t3014: F, t6205: F, t2926: F, t6141: F, t342: F, t6343: F, t6271: F, t73: F, t11249: F, t6305: F, t6234: F, t993: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t19133, t19153, t19156, t19173, t19255, t19275) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1059::<F>(t11465, t6189, t3336, t6396, t6184, t964, t6152, t945, t11387, t6109, t2970, t6173);
        let (t19303, t19330, t19351, t19446, t19450) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1060::<F>(t3014, t6205, t2926, t6141, t342, t6343, t6271, t73, t11249, t6305);
        let t19462 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1061::<F>(t6234, t993);
    (t19133, t19153, t19156, t19173, t19255, t19275, t19303, t19330, t19351, t19446, t19450, t19462)
}
