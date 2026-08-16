//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1126/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1126(t9858: f64, t9861: f64, t2619: f64, t5635: f64, t13664: f64, t13667: f64, t13669: f64, t13671: f64, t13673: f64, t13682: f64, t13683: f64, t9524: f64, t9542: f64, t9588: f64, t9854: f64, t9865: f64, t9868: f64) -> (f64, f64, f64, f64) {
    let t13885 = 0.34631718211362927518e2_f64 * t9858;
    let t13886 = 0.21687162600603479684e-1_f64 * t9861;
    let t13887 = t5635 * t2619;
    let t13888 = 0.24415263074675393405e-3_f64 * t13887;
    let t13889 = -t9588 - t9524 - t13664 + t13667 + t13669 - t13671 + t13673 + t9542 + t13682 - t9854 + t13683 - t13885 + t13886 + t9865 + t9868 + t13888;
    (t13885, t13886, t13888, t13889)
}
