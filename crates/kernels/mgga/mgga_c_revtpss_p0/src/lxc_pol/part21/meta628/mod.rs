//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2392;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2393;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta628<F: Float>(t2237: F, t2482: F, t849: F, t2677: F, t10489: F, t221: F, t2674: F, t2675: F, t234: F, t9801: F, t10887: F, t136: F, t2475: F, t220: F, t10777: F, t2731: F, t837: F, t2668: F, t823: F, t10782: F, t159: F, t33127: F, t64: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40710, t40711, t40719, t40721, t40722, t40724) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2392::<F>(t2237, t2482, t849, t2677, t10489, t221, t2674, t2675, t234, t9801, t10887, t136, t2475);
        let (t40725, t40728, t40731, t40732, t40735) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2393::<F>(t220, t40724, t10777, t2731, t837, t2482, t2668, t823, t10782, t159, t33127, t64);
    (t40710, t40711, t40719, t40721, t40722, t40724, t40725, t40728, t40731, t40732, t40735)
}
