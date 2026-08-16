//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta317 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1606;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta317<F: Float>(t1514: F, t2289: F, t4264: F, t625: F, t4288: F, t2339: F, t4287: F, t2349: F, t97: F, t105: F, t2357: F, t1468: F, t9335: F) -> (F, F, F, F, F, F, F) {
        let (t13448, t13451, t13453, t13458, t13475, t13496, t13550) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1606::<F>(t1514, t2289, t4264, t625, t4288, t2339, t4287, t2349, t97, t105, t2357, t1468, t9335);
    (t13448, t13451, t13453, t13458, t13475, t13496, t13550)
}
