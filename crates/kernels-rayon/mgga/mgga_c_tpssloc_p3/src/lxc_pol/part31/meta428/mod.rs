//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta428 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1554;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta428(t22814: f64, t22816: f64, t1999: f64, t794: f64, t61: f64, t9222: f64, t1995: f64, t133: f64, t6933: f64, t6604: f64, t6925: f64, t242: f64, t6943: f64, t1336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22818, t22819, t22822, t22823, t22825, t22827) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1554(t22814, t22816, t1999, t794, t61, t9222, t1995, t133, t6933, t6604, t6925);
        let (t22832, t22833) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1555(t242, t6943, t1336);
    (t22818, t22819, t22822, t22823, t22825, t22827, t22832, t22833)
}
