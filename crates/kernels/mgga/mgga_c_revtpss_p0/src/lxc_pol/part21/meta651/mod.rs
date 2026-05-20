//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta651 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2437;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2438;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta651<F: Float>(t1032: F, t1040: F, t11902: F, t11762: F, t3241: F, t11752: F, t11755: F, t1011: F, t3247: F, t697: F, t3254: F, t11789: F, t11937: F, t225: F, t42051: F, t11783: F, t3215: F, t11817: F, t3211: F, t1025: F, t1026: F, t2434: F, t371: F, t11901: F, t993: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t42235, t42240, t42249, t42251, t42254, t42257, t42259) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2437::<F>(t1032, t1040, t11902, t11762, t3241, t11752, t11755, t1011, t3247, t697, t3254, t11789, t11937);
        let (t42261, t42268, t42270, t42274, t42277) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2438::<F>(t225, t42051, t11783, t3215, t11817, t3211, t1025, t1026, t2434, t371, t11901, t993);
    (t42235, t42240, t42249, t42251, t42254, t42257, t42259, t42261, t42268, t42270, t42274, t42277)
}
