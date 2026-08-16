//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta258 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta258(t1719: f64, t3432: f64, t1729: f64, t2439: f64, t1737: f64, t3451: f64, t3476: f64, t3383: f64, t1749: f64, t3520: f64, t3495: f64, t1770: f64, t3781: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16840, t16876, t17023, t17032, t17092, t17097, t17154, t17183) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1026(t1719, t3432, t1729, t2439, t1737, t3451, t3476, t3383, t1749, t3520, t3495, t1770, t3781);
    (t16840, t16876, t17023, t17032, t17092, t17097, t17154, t17183)
}
