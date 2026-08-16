//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta280 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1511;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta280(t10076: f64, t10145: f64, t1427: f64, t1357: f64, t4078: f64, t689: f64, t1445: f64, t3899: f64, t10115: f64, t562: f64, t2435: f64, t3903: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10146, t10147, t10150, t10151, t10153, t10154, t10157, t10160) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1511(t10076, t10145, t1427, t1357, t4078, t689, t1445, t3899, t10115, t562, t2435, t3903);
    (t10146, t10147, t10150, t10151, t10153, t10154, t10157, t10160)
}
