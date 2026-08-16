//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta687 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2505;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2506;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta687<F: Float>(t221: F, t461: F, t462: F, t624: F, t1250: F, t606: F, t1235: F, t3661: F, t371: F, t676: F, t1236: F, t2434: F, t1208: F, t12689: F, t225: F, t480: F, t3671: F, t3672: F, t12625: F, t458: F, t456: F, t43813: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t44797, t44799, t44823, t44829) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2505::<F>(t221, t461, t462, t624, t1250, t606, t1235, t3661, t371, t676, t1236, t2434);
        let (t44831, t44832, t44833, t44838, t44842, t44843, t44865) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2506::<F>(t1208, t12689, t225, t480, t3671, t3672, t371, t676, t12625, t458, t456, t43813);
    (t44797, t44799, t44823, t44829, t44831, t44832, t44833, t44838, t44842, t44843, t44865)
}
