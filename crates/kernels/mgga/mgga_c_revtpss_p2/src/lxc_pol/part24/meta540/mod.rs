//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1587;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1588;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta540<F: Float>(t22912: F, t4101: F, t686: F, t72: F, t543: F, t85659: F, t2782: F, t4100: F, t4003: F, t5744: F, t86445: F, t4086: F, t86441: F, t1904: F, t22445: F, t689: F, t22974: F, t47603: F, t213: F, t22964: F, t13729: F, t556: F, t6918: F) -> (F, F, F, F, F, F, F, F) {
        let (t86639, t86643, t86647, t86654) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1587::<F>(t22912, t4101, t686, t72, t543, t85659, t2782, t4100, t4003, t5744, t86445, t4086, t86441);
        let (t86682, t86699, t86701, t86712) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1588::<F>(t1904, t22445, t689, t22974, t47603, t686, t72, t213, t22964, t13729, t2782, t556, t6918);
    (t86639, t86643, t86647, t86654, t86682, t86699, t86701, t86712)
}
