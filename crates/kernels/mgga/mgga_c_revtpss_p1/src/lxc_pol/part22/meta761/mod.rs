//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta761 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2842;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2843;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta761<F: Float>(t2857: F, t3154: F, t2251: F, t11262: F, t3127: F, t3129: F, t11988: F, t3106: F, t271: F, t2852: F, t1054: F, t11970: F, t11986: F, t828: F, t3091: F, t3096: F, t12097: F, t3090: F, t11273: F, t12012: F, t11631: F, t3133: F, t1086: F, t11223: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t43174, t43175, t43204, t43215, t43222, t43238) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2842::<F>(t2857, t3154, t2251, t11262, t3127, t3129, t11988, t3106, t271, t2852, t1054, t11970);
        let (t43240, t43242, t43244, t43268, t43279, t43285) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2843::<F>(t11986, t828, t3091, t3096, t12097, t3090, t11273, t12012, t11631, t3133, t1086, t11223);
    (t43174, t43175, t43204, t43215, t43222, t43238, t43240, t43242, t43244, t43268, t43279, t43285)
}
