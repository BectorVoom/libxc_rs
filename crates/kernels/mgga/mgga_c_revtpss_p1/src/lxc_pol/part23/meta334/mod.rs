//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta334 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1634;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta334<F: Float>(t14127: F, t4086: F, t543: F, t2782: F, t1882: F, t4114: F, t2482: F, t122: F, t4003: F, t72: F, t1398: F, t676: F) -> (F, F, F, F, F, F) {
        let (t14129, t14131, t14140, t14141, t14143, t14144) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1634::<F>(t14127, t4086, t543, t2782, t1882, t4114, t2482, t122, t4003, t72, t1398, t676);
    (t14129, t14131, t14140, t14141, t14143, t14144)
}
