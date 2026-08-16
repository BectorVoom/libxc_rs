//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta169 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk900;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk901;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk902;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta169(t2495: f64, t9368: f64, t9417: f64, t1340: f64, t2626: f64, t4038: f64, t2491: f64, t745: f64, t1330: f64, t2608: f64, t512: f64, t169: f64, t2552: f64, t164: f64, t2538: f64, t729: f64, t2556: f64, t9283: f64, t9286: f64, t9289: f64, t9292: f64, t9296: f64, t9298: f64, t9300: f64, t9303: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t9419 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk900(t2495, t9368, t9417);
        let (t9421, t9423, t9425) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk901(t1340, t9419, t2626, t4038, t2491, t745, t9368);
        let (t9427, t9428, t9430, t9432, t9433, t9434, t9435, t9446) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk902(t1340, t9425, t1330, t2608, t512, t169, t2552, t164, t2538, t729, t2556, t9283, t9286, t9289, t9292, t9296, t9298, t9300, t9303);
    (t9419, t9421, t9423, t9425, t9427, t9428, t9430, t9432, t9433, t9434, t9435, t9446)
}
