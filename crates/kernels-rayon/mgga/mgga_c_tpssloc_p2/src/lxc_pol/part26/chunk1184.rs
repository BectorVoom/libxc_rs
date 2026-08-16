//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1184/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1184(t1393: f64, t2114: f64, t22577: f64, t22580: f64, t22583: f64, t22587: f64, t22594: f64, t22599: f64, t22605: f64, t22608: f64, t22610: f64, t22612: f64, t22614: f64, t22616: f64, t22618: f64, t22950: f64, t23833: f64, t23835: f64, t23837: f64, t23860: f64, t3652: f64, t7412: f64) -> f64 {
    let t24953 = 2.0_f64 * t1393 * t7412 - t2114 * t3652 - t22577 - t22580 - t22583 + t22587 + t22594 + t22599 + t22605 + t22608 - t22610 - t22612 - t22614 - t22616 - t22618 + t22950 - t23833 - t23835 + t23837 + t23860;
    t24953
}
