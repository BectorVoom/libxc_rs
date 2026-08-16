//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2777/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2777(t22041: f64, t3957: f64, t2661: f64, t74026: f64, t9835: f64, t9934: f64, t22016: f64, t22025: f64, t46609: f64, t6846: f64, t9909: f64, t1399: f64, t22236: f64, t3992: f64) -> (f64, f64, f64, f64, f64) {
    let t74547 = t3957 * t22041;
    let t74579 = t2661 * t9934 * t74026 * t9835;
    let t74583 = t2661 * t46609 * t22025 * t22016;
    let t74585 = t9909 * t6846;
    let t74589 = t2661 * t3992 * t22236 * t1399;
    (t74547, t74579, t74583, t74585, t74589)
}
