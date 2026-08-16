//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1996;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1997;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta403<F: Float>(t14066: F, t225: F, t5774: F, t72: F, t686: F, t3915: F, t5711: F, t786: F, t1364: F, t1357: F, t5775: F, t689: F, t213: F, t4071: F, t561: F, t5728: F, t9666: F, t9668: F, t9672: F, t9677: F, t9683: F, t9687: F, t9691: F, t9694: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14067, t14078, t14079, t14081, t14082, t14084, t14085, t14087) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1996::<F>(t14066, t225, t5774, t72, t686, t3915, t5711, t786, t1364, t1357, t5775, t689);
        let t14088 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1997::<F>(t14067, t14081, t14084, t14087, t213, t4071, t561, t5728, t9666, t9668, t9672, t9677, t9683, t9687, t9691, t9694);
    (t14067, t14078, t14079, t14081, t14082, t14084, t14085, t14087, t14088)
}
