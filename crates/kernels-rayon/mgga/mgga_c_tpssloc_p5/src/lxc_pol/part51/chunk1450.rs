//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1450/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1450(t33245: f64, t6897: f64, t794: f64, t1985: f64, t26202: f64, t31611: f64, t115658: f64, t120641: f64, t120649: f64, t1375: f64, t16030: f64, t1842: f64, t2016: f64, t24095: f64, t26348: f64, t26371: f64, t31564: f64, t31641: f64, t33323: f64, t3887: f64, t5321: f64, t6992: f64, t7194: f64, t7729: f64, t7936: f64, t8627: f64, t90665: f64, t93338: f64) -> f64 {
    let t122551 = t6897 * t794 * t33245;
    let t122562 = t1985 * t31611 * t26202;
    let t122576 = -6.0_f64 * t90665 * t33323 - t120641 - 0.41123351671205660912e-2_f64 * t122551 + 2.0_f64 * t16030 * t8627 + 2.0_f64 * t1375 * t3887 * t31641 * t1842 - 0.41123351671205660912e-2_f64 * t115658 - t93338 * t2016 - 0.82246703342411321825e-2_f64 * t122562 - t120649 + 2.0_f64 * t7194 * t26348 + 2.0_f64 * t7194 * t26371 + 2.0_f64 * t24095 * t7729 + 2.0_f64 * t1375 * t3887 * t7936 * t6992 + 2.0_f64 * t5321 * t31564;
    t122576
}
