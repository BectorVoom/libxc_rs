//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta407 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2003;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta407<F: Float>(t122: F, t4003: F, t72: F, t1398: F, t676: F, t14141: F, t10069: F, t5737: F, t10015: F, t10020: F, t10027: F, t10032: F, t10035: F, t10041: F, t10044: F, t14116: F, t14120: F, t14126: F, t14131: F, t4004: F, t5735: F, t5745: F, t9840: F) -> (F, F, F, F, F, F) {
        let (t14143, t14144, t14145) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2003::<F>(t122, t4003, t72, t1398, t676);
        let (t14146, t14149, t14151) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2004::<F>(t14141, t14145, t10069, t5737, t10015, t10020, t10027, t10032, t10035, t10041, t10044, t14116, t14120, t14126, t14131, t4004, t5735, t5745, t9840);
    (t14143, t14144, t14145, t14146, t14149, t14151)
}
