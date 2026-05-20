//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta666 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2464;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2465;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta666<F: Float>(t3075: F, t3154: F, t11671: F, t11865: F, t11697: F, t11710: F, t3091: F, t11725: F, t828: F, t11706: F, t11779: F, t3215: F, t225: F, t42059: F, t11675: F, t11711: F, t11666: F, t4899: F, t11262: F, t3127: F, t3129: F, t11630: F, t11633: F, t3172: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t43116, t43121, t43129, t43131, t43133, t43146) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2464::<F>(t3075, t3154, t11671, t11865, t11697, t11710, t3091, t11725, t828, t11706, t11779, t3215);
        let (t43154, t43169, t43172, t43204, t43211) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2465::<F>(t225, t42059, t11675, t11711, t11666, t11710, t4899, t11262, t3127, t3129, t11630, t11633, t3172);
    (t43116, t43121, t43129, t43131, t43133, t43146, t43154, t43169, t43172, t43204, t43211)
}
