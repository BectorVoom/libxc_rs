//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta798 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2890;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2891;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2892;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta798<F: Float>(t15475: F, t2869: F, t11385: F, t1609: F, t11387: F, t2918: F, t934: F, t41578: F, t4636: F, t11528: F, t15380: F, t11294: F, t15390: F, t2874: F, t15474: F, t2924: F, t2926: F, t11300: F, t4635: F, t2873: F, t4587: F, t2876: F, t11298: F, t1596: F, t11301: F, t11466: F, t1633: F, t11299: F, t11116: F, t11525: F, t11551: F, t11557: F, t15350: F, t15406: F, t52137: F, t965: F, t973: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t52481, t52486, t52488, t52490, t52492) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2890::<F>(t15475, t2869, t11385, t1609, t11387, t2918, t934, t41578, t4636, t11528, t15380, t11294, t15390);
        let (t52495, t52499, t52502, t52507) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2891::<F>(t15475, t2874, t934, t15474, t2924, t2926, t11300, t11385, t4635, t2873, t4587, t2876);
        let (t52510, t52516, t52520) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2892::<F>(t11298, t1596, t11301, t11466, t1633, t11299, t1609, t11116, t11525, t11551, t11557, t15350, t15406, t52137, t52481, t52486, t52488, t52490, t52492, t52495, t52499, t52502, t52507, t965, t973);
    (t52481, t52486, t52488, t52490, t52492, t52495, t52499, t52502, t52507, t52510, t52516, t52520)
}
