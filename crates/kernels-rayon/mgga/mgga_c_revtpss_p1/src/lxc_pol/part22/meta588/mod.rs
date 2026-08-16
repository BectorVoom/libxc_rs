//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2461;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2462;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta588(t1587: f64, t2: f64, t580: f64, t11506: f64, t6189: f64, t11509: f64, t972: f64, t981: f64, t11144: f64, t5819: f64, t606: f64, t11142: f64, t128: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18890, t18892, t18898, t18900, t18902, t18903, t18904, t18905) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2461(t1587, t2, t580, t11506, t6189, t11509, t972, t981, t11144, t5819, t606, t11142);
        let t18906 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2462(t128, t18905);
    (t18890, t18892, t18898, t18900, t18902, t18903, t18904, t18905, t18906)
}
