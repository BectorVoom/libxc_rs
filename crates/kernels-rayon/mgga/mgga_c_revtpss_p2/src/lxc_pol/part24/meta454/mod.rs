//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1421;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta454(t15014: f64, t9303: f64, t10982: f64, t1568: f64, t9646: f64, t14986: f64, t2453: f64, t14567: f64, t14557: f64, t4519: f64, t9292: f64, t2798: f64, t4499: f64, t9288: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t51237, t51246, t51258, t51297, t51390, t51403, t51408) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1421(t15014, t9303, t10982, t1568, t9646, t14986, t2453, t14567, t14557, t4519, t9292, t2798, t4499, t9288);
    (t51237, t51246, t51258, t51297, t51390, t51403, t51408)
}
