//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta671 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2472;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta671(t3057: f64, t4995: f64, t3143: f64, t42859: f64, t342: f64, t12032: f64, t359: f64, t3043: f64, t3298: f64, t16551: f64, t994: f64, t16558: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t43456, t43471, t43472, t43504, t43512, t43520, t43524) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2472(t3057, t4995, t3143, t42859, t342, t12032, t359, t3043, t3298, t16551, t994, t16558);
    (t43456, t43471, t43472, t43504, t43512, t43520, t43524)
}
