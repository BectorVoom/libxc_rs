//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta271 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1489;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta271<F: Float>(t240: F, t9948: F, t247: F, t550: F, t548: F, t4010: F, t72: F, t245: F, t3829: F, t543: F, t3937: F, t1386: F, t820: F, t844: F) -> (F, F, F, F, F, F, F) {
        let (t9949, t9953, t9954, t9955, t9956) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1489::<F>(t240, t9948, t247, t550, t548, t4010, t72, t245, t3829, t543);
        let (t9958, t9962) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1490::<F>(t3937, t9955, t9956, t1386, t820, t844);
    (t9949, t9953, t9954, t9955, t9956, t9958, t9962)
}
