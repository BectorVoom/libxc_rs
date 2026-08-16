//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2368;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2369;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta616(t40196: f64, t760: f64, t10587: f64, t2626: f64, t2523: f64, t9425: f64, t2389: f64, t37: f64, t2394: f64, t2475: f64, t10069: f64, t10929: f64, t138: f64, t785: f64, t9302: f64, t2786: f64, t10073: f64, t10920: f64, t231: f64, t2760: f64, t2782: f64, t2783: f64, t836: f64, t10871: f64, t14545: f64, t39709: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40198, t40203, t40205, t40207, t40236, t40267) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2368(t40196, t760, t10587, t2626, t2523, t9425, t2389, t37, t2394, t2475, t10069, t10929);
        let (t40270, t40271, t40273, t40278, t40282) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2369(t138, t785, t9302, t2786, t10073, t10920, t231, t2760, t2782, t2783, t836, t10871, t14545, t39709);
    (t40198, t40203, t40205, t40207, t40236, t40267, t40270, t40271, t40273, t40278, t40282)
}
