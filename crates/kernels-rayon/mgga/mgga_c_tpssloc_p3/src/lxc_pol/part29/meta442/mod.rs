//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1747;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1748;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1749;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta442(t116: f64, t117: f64, t67: f64, t22814: f64, t1999: f64, t794: f64, t61: f64, t9222: f64, t1995: f64, t133: f64, t6933: f64, t6604: f64, t6925: f64, t16312: f64, t550: f64, t1339: f64, t242: f64, t6943: f64, t1336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t22816 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1747(t116, t117, t67);
        let (t22818, t22820, t22822, t22823, t22826, t22827) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1748(t22814, t22816, t1999, t794, t61, t9222, t1995, t133, t6933, t6604, t6925);
        let (t22828, t22829, t22830, t22832, t22833) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1749(t16312, t550, t1339, t22827, t242, t6943, t1336);
    (t22816, t22818, t22820, t22822, t22823, t22826, t22827, t22828, t22829, t22830, t22832, t22833)
}
