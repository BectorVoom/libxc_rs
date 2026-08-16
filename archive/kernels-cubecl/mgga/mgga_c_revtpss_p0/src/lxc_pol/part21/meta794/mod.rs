//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta794 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2873;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2874;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta794<F: Float>(t15389: F, t2918: F, t2924: F, t11387: F, t4631: F, t11385: F, t2875: F, t51840: F, t51844: F, t51846: F, t52141: F, t52146: F, t52150: F, t52153: F, t52156: F, t52159: F, t11379: F, t4635: F, t11300: F, t1609: F, t41499: F, t41502: F, t11528: F, t15383: F, t15386: F, t41883: F, t11294: F, t15393: F) -> (F, F, F, F, F, F, F, F) {
        let (t52162, t52166, t52167) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2873::<F>(t15389, t2918, t2924, t11387, t4631, t11385, t2875, t51840, t51844, t51846, t52141, t52146, t52150, t52153, t52156, t52159);
        let (t52170, t52174, t52176, t52178, t52180) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2874::<F>(t11379, t2924, t4635, t11300, t1609, t41499, t41502, t11528, t15383, t15386, t41883, t11294, t15393);
    (t52162, t52166, t52167, t52170, t52174, t52176, t52178, t52180)
}
