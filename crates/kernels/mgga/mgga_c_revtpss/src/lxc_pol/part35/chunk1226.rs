//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1226/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1226<F: Float>(t115493: F, t115521: F, t115551: F, t115592: F, t115614: F, t115637: F, t115658: F, t115744: F, t892: F, t102888: F, t103586: F, t113097: F, t113100: F, t113104: F, t113108: F, t113115: F, t113123: F, t113428: F, t113433: F, t113441: F, t1940: F, t2071: F, t2072: F, t2403: F, t26425: F, t28291: F, t28472: F, t29591: F, t29599: F, t29602: F, t29606: F, t29713: F, t30: F, t4541: F, t8020: F) -> (F, F, F) {
    let t115747 = t115493 + t115521 + t115551 + t115592 + t115614 + t115637 + t115658 + t115744;
    let t115748 = t115747 * t892;
    let t115763 = -F::new(9.0) * t102888 * t29599 + F::new(3.0) * t28472 * t113108 + F::new(9.0) * t2403 * t8020 * t29602 + F::new(3.0) * t1940 * t103586 * t29713 - F::new(9.0) / F::new(2.0) * t26425 * t113104 - F::new(9.0) * t26425 * t113115 - F::new(9.0) / F::new(2.0) * t26425 * t113433 + F::new(3.0) * t113123 * t2072 - F::new(9.0) * t28291 * t113097 + F::new(9.0) * t28291 * t113100 + t1940 * t115748 * t30 / F::new(2.0) + F::new(9.0) / F::new(2.0) * t2403 * t8020 * t29606 + F::new(9.0) * t26425 * t113441 + F::new(3.0) / F::new(2.0) * t2403 * t2071 * t113428 + F::new(9.0) * t4541 * t8020 * t29591;
    (t115747, t115748, t115763)
}
