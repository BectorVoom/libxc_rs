//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta430 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1380;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta430(t45936: f64, t584: f64, t596: f64, t20: f64, t2237: f64, t12: f64, t14: f64, t27: f64, t10285: f64, t2231: f64, t10293: f64, t592: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t45937, t45939, t45941, t45944, t45946, t45948, t45949) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1380(t45936, t584, t596, t20, t2237, t12, t14, t27, t10285, t2231, t10293, t592);
    (t45937, t45939, t45941, t45944, t45946, t45948, t45949)
}
