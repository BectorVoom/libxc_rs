//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta693 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2514;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2515;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta693<F: Float>(t12378: F, t300: F, t13062: F, t13064: F, t3172: F, t1247: F, t13075: F, t1209: F, t13126: F, t17708: F, t127: F, t12988: F, t12989: F, t371: F, t1203: F, t12626: F, t225: F, t12967: F, t12995: F, t12627: F, t1269: F, t3566: F, t3727: F, t12640: F, t44842: F, t487: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t45319, t45346, t45352, t45371, t45382) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2514::<F>(t12378, t300, t13062, t13064, t3172, t1247, t13075, t1209, t13126, t17708, t127, t12988, t12989, t371);
        let (t45384, t45385, t45389, t45427, t45430, t45433, t45438) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2515::<F>(t1203, t12626, t225, t12967, t12995, t12627, t1269, t3566, t3727, t12640, t44842, t487);
    (t45319, t45346, t45352, t45371, t45382, t45384, t45385, t45389, t45427, t45430, t45433, t45438)
}
