//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 994/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk994(t2523: f64, t2626: f64, t760: f64, t9425: f64, t2609: f64, t606: f64, t706: f64, t775: f64, t853: f64, t2710: f64, t2793: f64, t9285: f64) -> (f64, f64, f64, f64, f64) {
    let t10608 = t2523 * t2626;
    let t10611 = 0.35089341735807877242e1_f64 * t760 * t9425;
    let t10612 = t2609 * t606;
    let t10613 = t706 * t10612;
    let t10631 = t853 * t775;
    let t10645 = 0.46263278077393568556e-2_f64 * t2710 * t2793 * t9285;
    (t10608, t10611, t10613, t10631, t10645)
}
