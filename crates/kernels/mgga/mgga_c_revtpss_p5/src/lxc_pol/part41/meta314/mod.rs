//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta314 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1085;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1086;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta314<F: Float>(t1514: F, t2289: F, t4264: F, t625: F, t4288: F, t2349: F, t97: F, t105: F, t2357: F, t1857: F, t3857: F, t177: F, t5566: F, t762: F, t1450: F, t5778: F, t2516: F, t5571: F, t72: F, t757: F, t1320: F, t5567: F, t5569: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13448, t13451, t13453, t13475, t13496, t13584, t13597) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1085::<F>(t1514, t2289, t4264, t625, t4288, t2349, t97, t105, t2357, t1857, t3857, t177, t5566);
        let (t13599, t13600, t13611, t13615, t13620, t13621) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1086::<F>(t13597, t762, t1450, t5778, t2516, t5571, t5566, t72, t757, t1320, t5567, t5569);
    (t13448, t13451, t13453, t13475, t13496, t13584, t13599, t13600, t13611, t13615, t13620, t13621)
}
