//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1403;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1404;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta444(t1427: f64, t1903: f64, t22: f64, t9647: f64, t14296: f64, t9303: f64, t5718: f64, t9292: f64, t14099: f64, t2453: f64, t5603: f64, t9692: f64, t3915: f64, t5721: f64, t9288: f64, t14293: f64, t9664: f64, t14103: f64, t9285: f64, t9674: f64, t13726: f64, t10115: f64, t1900: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47781, t47786, t47802, t47856, t47863) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1403(t1427, t1903, t22, t9647, t14296, t9303, t5718, t9292, t14099, t2453, t5603, t9692);
        let (t47904, t47920, t47932, t47938, t47961) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1404(t3915, t5721, t9288, t14293, t9664, t14103, t9285, t9674, t13726, t9303, t10115, t1900);
    (t47781, t47786, t47802, t47856, t47863, t47904, t47920, t47932, t47938, t47961)
}
