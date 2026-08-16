//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1130/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1130(t4057: f64, t5673: f64, t5674: f64, t13848: f64, t3938: f64, t9818: f64, t9816: f64, t125: f64, t5658: f64, t1399: f64, t2689: f64, t5618: f64) -> (f64, f64, f64, f64, f64) {
    let t13937 = t5673 * t5674 * t4057;
    let t13941 = t9818 * t13848 * t3938;
    let t13943 = 0.10164000561857065645e-3_f64 * t9816 * t13941;
    let t13944 = t125 * t5658;
    let t13946 = t5673 * t13944 * t1399;
    let t13949 = t2689 * t5618;
    (t13937, t13943, t13944, t13946, t13949)
}
