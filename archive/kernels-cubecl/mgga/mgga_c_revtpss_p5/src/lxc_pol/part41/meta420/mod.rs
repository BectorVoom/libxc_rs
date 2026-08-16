//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1475;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1476;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta420<F: Float>(t2178: F, t6765: F, t6934: F, t5891: F, t8259: F, t1504: F, t1513: F, t8268: F, t5915: F, t31058: F, t5895: F, t5823: F, t114: F, t31026: F, t31035: F, t31259: F, t31274: F, t8258: F, t8267: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t31518, t31533, t31538, t31541, t31542, t31545, t31548, t31551) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1475::<F>(t2178, t6765, t6934, t5891, t8259, t1504, t1513, t8268, t5915, t31058, t5895, t5823);
        let t31555 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1476::<F>(t114, t31026, t31035, t31259, t31274, t31538, t31542, t31545, t31548, t31551, t8258, t8267);
    (t31518, t31533, t31538, t31541, t31542, t31545, t31548, t31551, t31555)
}
