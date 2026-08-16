//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1008/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1008(t10605: f64, t2612: f64, t2523: f64, t2626: f64, t760: f64, t9425: f64, t2609: f64, t606: f64, t706: f64, t10592: f64, t10594: f64, t10596: f64, t10598: f64, t10602: f64, t10604: f64, t9542: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10607 = 36.0_f64 * t10605 * t2612;
    let t10608 = t2523 * t2626;
    let t10609 = 0.35089341735807877242e1_f64 * t10608;
    let t10611 = 0.35089341735807877242e1_f64 * t760 * t9425;
    let t10612 = t2609 * t606;
    let t10613 = t706 * t10612;
    let t10614 = 12.0_f64 * t10613;
    let t10615 = t10592 - t10594 - t10596 - t10598 + t10602 - t10604 + t9542 + t10607 + t10609 - t10611 + t10614;
    (t10607, t10609, t10611, t10612, t10614, t10615)
}
