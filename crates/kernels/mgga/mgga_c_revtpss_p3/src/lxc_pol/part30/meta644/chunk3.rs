//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2264/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2264<F: Float>(t104135: F, t104153: F, t104433: F, t105712: F, t105724: F, t105734: F, t105741: F, t105756: F, t101568: F, t101570: F, t101572: F, t101576: F, t101578: F, t101583: F, t101586: F, t101590: F, t101594: F, t101598: F, t101601: F, t101606: F, t18204: F, t18211: F, t2170: F, t4165: F, t573: F, t5805: F, t7696: F, t8245: F, param_d: F) -> (F, F) {
    let t105759 = t104135 + t104153 + t104433 + t105712 + t105724 + t105734 + t105741 + t105756;
    let t105762 = t105759 * t573 * param_d + F::new(6.0) * t18204 * t2170 + F::new(6.0) * t18211 * t2170 + F::new(3.0) * t4165 * t8245 + F::new(6.0) * t5805 * t7696 + t101568 + t101570 + t101572 + t101576 + t101578 + t101583 + t101586 + t101590 + t101594 + t101598 + t101601 + t101606;
    (t105759, t105762)
}
