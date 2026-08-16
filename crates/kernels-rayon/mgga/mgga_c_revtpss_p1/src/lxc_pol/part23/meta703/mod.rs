//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta703 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2453;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta703(t3853: f64, t3857: f64, t820: f64, t843: f64, t9991: f64, t1386: f64, t2237: f64, t2482: f64, t4021: f64, t235: f64, t46475: f64, t4000: f64, t596: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t47152, t47194, t47198, t47199, t47201, t47215) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2453(t3853, t3857, t820, t843, t9991, t1386, t2237, t2482, t4021, t235, t46475, t4000, t596);
    (t47152, t47194, t47198, t47199, t47201, t47215)
}
