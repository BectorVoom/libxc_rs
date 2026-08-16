//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta416 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1234;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1235;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta416(t13278: f64, t5619: f64, t1512: f64, t59281: f64, t67441: f64, t816: f64, t20978: f64, t9638: f64, t20938: f64, t838: f64, t20953: f64, t2639: f64, t20994: f64, t2563: f64, t20944: f64, t41011: f64, t5614: f64, t20963: f64, t9667: f64, t46881: f64, t5587: f64, t20908: f64, t2697: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t67852, t67854, t67872, t67880, t67882, t67884) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1234(t13278, t5619, t1512, t59281, t67441, t816, t20978, t9638, t20938, t838, t20953, t2639);
        let (t67920, t67937, t67976, t67978, t67980, t68021) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1235(t20994, t2563, t20944, t41011, t13278, t5614, t20963, t9667, t46881, t5587, t20908, t2697);
    (t67852, t67854, t67872, t67880, t67882, t67884, t67920, t67937, t67976, t67978, t67980, t68021)
}
