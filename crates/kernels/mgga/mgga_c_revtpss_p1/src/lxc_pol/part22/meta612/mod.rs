//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2516;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2517;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta612<F: Float>(t1678: F, t3316: F, t342: F, t6299: F, t73: F, t4976: F, t1082: F, t19414: F, t1045: F, t999: F) -> (F, F, F, F, F, F) {
        let (t19607, t19608, t19611) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2516::<F>(t1678, t3316, t342, t6299, t73);
        let (t19612, t19617, t19620) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2517::<F>(t19611, t4976, t1082, t19414, t1045, t999);
    (t19607, t19608, t19611, t19612, t19617, t19620)
}
