//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1060/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1060(t127539: f64, t128551: f64, t128552: f64, t128562: f64, t128564: f64, t128567: f64, t128571: f64, t128573: f64, t128575: f64, t128577: f64, t128581: f64, t128584: f64, t128588: f64, t128592: f64, t2165: f64, t28943: f64, t28969: f64, t33746: f64, t7904: f64, t7943: f64, t8690: f64) -> f64 {
    let t130472 = -t2165 * t28943 + 3.0_f64 * t28969 * t8690 + 6.0_f64 * t33746 * t7904 - 2.0_f64 * t33746 * t7943 - t127539 + t128551 - t128552 + t128562 + t128564 + t128567 + t128571 - t128573 - t128575 - t128577 - t128581 + t128584 + t128588 - t128592;
    t130472
}
