//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1853;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta542(t7399: f64, t786: f64, t867: f64, t93173: f64, t95725: f64, t93371: f64, t2453: f64, t26496: f64, t10506: f64, t10510: f64, t26497: f64, t10073: f64, t25402: f64, t7056: f64, t7398: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t95743, t95746, t95747, t95773, t95774, t95779, t95783) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1853(t7399, t786, t867, t93173, t95725, t93371, t2453, t26496, t10506, t10510, t26497, t10073, t25402, t7056, t7398);
    (t95743, t95746, t95747, t95773, t95774, t95779, t95783)
}
