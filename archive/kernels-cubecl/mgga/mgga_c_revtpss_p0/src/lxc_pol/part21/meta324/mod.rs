//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta324 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1605;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta324<F: Float>(t1100: F, t3333: F, t3335: F, t389: F, t2918: F, t936: F, t2874: F, t2926: F, t934: F, t2924: F, t1077: F, t225: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11105, t11108, t11112, t11114, t11116, t11118, t11119, t11120, t11121) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1605::<F>(t1100, t3333, t3335, t389, t2918, t936, t2874, t2926, t934, t2924, t1077, t225);
    (t11105, t11108, t11112, t11114, t11116, t11118, t11119, t11120, t11121)
}
