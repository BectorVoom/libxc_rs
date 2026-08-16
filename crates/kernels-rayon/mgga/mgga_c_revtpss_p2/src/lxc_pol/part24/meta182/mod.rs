//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta182 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk896;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk897;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta182(t1340: f64, t9419: f64, t2491: f64, t745: f64, t9368: f64, t169: f64, t2552: f64, t164: f64, t2538: f64, t729: f64, t2556: f64, t9283: f64, t9286: f64, t9289: f64, t9292: f64, t9296: f64, t9298: f64, t9300: f64, t9303: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9421, t9425) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk896(t1340, t9419, t2491, t745, t9368);
        let (t9427, t9432, t9433, t9434, t9435, t9446) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk897(t1340, t9425, t169, t2552, t164, t2538, t729, t2556, t9283, t9286, t9289, t9292, t9296, t9298, t9300, t9303);
    (t9421, t9425, t9427, t9432, t9433, t9434, t9435, t9446)
}
