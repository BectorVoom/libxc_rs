//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta117 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk651;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk652;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta117<F: Float>(t1514: F, t625: F, t1513: F, t2339: F, t1504: F, t2349: F, t1509: F, t2357: F, t1534: F, t72: F, t757: F, t1469: F, t750: F, t706: F, t1531: F, t705: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4261, t4263, t4269, t4279, t4302, t4303, t4305) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk651::<F>(t1514, t625, t1513, t2339, t1504, t2349, t1509, t2357, t1534, t72, t757, t1469, t750);
        let (t4306, t4311) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk652::<F>(t4305, t706, t1531, t705);
    (t4261, t4263, t4269, t4279, t4302, t4303, t4305, t4306, t4311)
}
