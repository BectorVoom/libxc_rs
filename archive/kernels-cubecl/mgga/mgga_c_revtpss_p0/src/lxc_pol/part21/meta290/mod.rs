//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1532;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1533;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta290<F: Float>(t10414: F, t117: F, t116: F, t2319: F, t10194: F, t10259: F, t1312: F, t2322: F, t2371: F, t5523: F, t670: F, t2389: F, t705: F) -> (F, F, F, F) {
        let (t10415, t10416) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1532::<F>(t10414, t117, t116, t2319);
        let (t10426, t10428) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1533::<F>(t10194, t10259, t10415, t10416, t1312, t2322, t2371, t5523, t670, t2389, t705);
    (t10415, t10416, t10426, t10428)
}
