//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1475;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1476;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta420(t2178: f64, t6765: f64, t6934: f64, t5891: f64, t8259: f64, t1504: f64, t1513: f64, t8268: f64, t5915: f64, t31058: f64, t5895: f64, t5823: f64, t114: f64, t31026: f64, t31035: f64, t31259: f64, t31274: f64, t8258: f64, t8267: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t31518, t31533, t31538, t31541, t31542, t31545, t31548, t31551) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1475(t2178, t6765, t6934, t5891, t8259, t1504, t1513, t8268, t5915, t31058, t5895, t5823);
        let t31555 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1476(t114, t31026, t31035, t31259, t31274, t31538, t31542, t31545, t31548, t31551, t8258, t8267);
    (t31518, t31533, t31538, t31541, t31542, t31545, t31548, t31551, t31555)
}
