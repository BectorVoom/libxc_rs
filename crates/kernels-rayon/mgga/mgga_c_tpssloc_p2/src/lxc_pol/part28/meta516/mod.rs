//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta516 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1764;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta516(t13487: f64, t57911: f64, t1484: f64, t2749: f64, t4303: f64, t868: f64, t4119: f64, t4233: f64, t829: f64, t16935: f64, t828: f64, t2745: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t57912, t57921, t58009, t58071, t58300, t58345, t59580) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1764(t13487, t57911, t1484, t2749, t4303, t868, t4119, t4233, t829, t16935, t828, t2745);
    (t57912, t57921, t58009, t58071, t58300, t58345, t59580)
}
