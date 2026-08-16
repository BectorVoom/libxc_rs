//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta258 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1094;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1095;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1096;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta258(t11428: f64, t11443: f64, t954: f64, t2966: f64, t944: f64, t302: f64, t2969: f64, t310: f64, t11410: f64, t2979: f64, t964: f64, t3011: f64, t960: f64, t3010: f64, t320: f64, t315: f64, t2988: f64, t972: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11444, t11445, t11449, t11450, t11452, t11453, t11456, t11461) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1094(t11428, t11443, t954, t2966, t944, t302, t2969, t310, t11410, t2979, t964, t3011, t960);
        let t11465 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1095(t3010, t320);
        let (t11466, t11467) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1096(t11465, t315, t2988, t972);
    (t11444, t11445, t11449, t11450, t11452, t11453, t11456, t11461, t11465, t11466, t11467)
}
