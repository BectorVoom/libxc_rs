//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta735 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2584;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2585;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta735<F: Float>(t3906: F, t3907: F, t39494: F, t1426: F, t4067: F, t786: F, t3917: F, t2453: F, t3908: F, t10115: F, t1421: F, t10168: F, t3920: F, t10174: F, t9676: F, t123: F, t2434: F, t3915: F, t4131: F, t10175: F, t9686: F, t1420: F, t4075: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t47504, t47506, t47507, t47510, t47512, t47516) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2584::<F>(t3906, t3907, t39494, t1426, t4067, t786, t3917, t2453, t3908, t10115, t1421, t10168, t3920);
        let (t47520, t47521, t47525, t47527, t47530) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2585::<F>(t10174, t2453, t9676, t123, t2434, t3915, t4131, t10175, t9686, t1420, t4075, t786);
    (t47504, t47506, t47507, t47510, t47512, t47516, t47520, t47521, t47525, t47527, t47530)
}
