//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2382;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2383;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta623(t10752: f64, t10905: f64, t2783: f64, t9801: f64, t10745: f64, t2735: f64, t4503: f64, t10728: f64, t808: f64, t10680: f64, t2710: f64, t2713: f64, t10732: f64, t10744: f64, t10674: f64, t2693: f64, t9732: f64, t14917: f64, t2475: f64, t2661: f64, t2662: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40511, t40517, t40518, t40523, t40526) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2382(t10752, t10905, t2783, t9801, t10745, t2735, t4503, t10728, t808, t10680, t2710, t2713);
        let (t40529, t40532, t40535, t40549) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2383(t10732, t10744, t808, t10674, t2710, t2713, t2693, t9732, t14917, t2475, t2661, t2662, t836);
    (t40511, t40517, t40518, t40523, t40526, t40529, t40532, t40535, t40549)
}
