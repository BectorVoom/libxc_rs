//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2054/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2054(t40889: f64, t68: f64, t852: f64, t9971: f64, t233: f64, t9970: f64, t2632: f64, t10021: f64, t812: f64, t841: f64, t849: f64, t23076: f64, t241: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40890 = t68 * t40889;
    let t40917 = t9971 * t852;
    let t40931 = 1.0_f64 / t9970 / t233;
    let t40933 = t2632 * t2632;
    let t40965 = t812 * t841 * t10021;
    let t40966 = t40965 * t849;
    let t40971 = t241 * t23076 * t67;
    (t40890, t40917, t40931, t40933, t40965, t40966, t40971)
}
