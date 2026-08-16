//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta193 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk953;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk954;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta193(t828: f64, t9400: f64, t9942: f64, t595: f64, t66: f64, t240: f64, t247: f64, t550: f64, t548: f64, t4010: f64, t72: f64, t245: f64, t3829: f64, t543: f64, t3937: f64, t1386: f64, t820: f64, t844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9944, t9948) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk953(t828, t9400, t9942, t595, t66);
        let (t9949, t9953, t9954, t9955, t9956, t9958, t9962) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk954(t240, t9948, t247, t550, t548, t4010, t72, t245, t3829, t543, t3937, t1386, t820, t844);
    (t9944, t9948, t9949, t9953, t9954, t9955, t9956, t9958, t9962)
}
