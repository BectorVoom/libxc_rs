//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta595 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1976;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1977;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta595(t12328: f64, t2003: f64, t12248: f64, t59: f64, t1336: f64, t2690: f64, t6943: f64, t1354: f64, t22865: f64, t6604: f64, t6937: f64, t22811: f64, t61: f64, t133: f64, t1995: f64, t6933: f64, t22803: f64, t2229: f64, t583: f64, t60: f64, t22816: f64, t22818: f64, t22764: f64, t3777: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t80900, t80901, t80914, t80915, t80939, t80940, t80953) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1976(t12328, t2003, t12248, t59, t1336, t2690, t6943, t1354, t22865, t6604, t6937, t22811, t61);
        let (t80957, t80958, t80967, t80971, t80991) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1977(t133, t1995, t6933, t80953, t22803, t6604, t2229, t583, t60, t22816, t22818, t22764, t3777);
    (t80900, t80901, t80914, t80915, t80939, t80940, t80953, t80957, t80958, t80967, t80971, t80991)
}
