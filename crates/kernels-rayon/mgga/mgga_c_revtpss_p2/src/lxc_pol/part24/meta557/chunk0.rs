//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1665/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1665(t23535: f64, t4598: f64, t18987: f64, t6120: f64, t4614: f64, t18979: f64, t11341: f64, t141: f64, t88116: f64, t88095: f64, t930: f64, t77804: f64, t88085: f64, t88093: f64, t88104: f64, t88108: f64, t88114: f64, t88122: f64, t88130: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t88220 = t4598 * t23535;
    let t88222 = t18987 * t6120;
    let t88224 = t4614 * t23535;
    let t88226 = t18979 * t6120;
    let t88229 = t141 * t11341 * t88116;
    let t88232 = t141 * t930 * t88095;
    let t88242 = -0.51785e1_f64 * t88220 - 0.247573125e0_f64 * t88222 + 0.3300975e0_f64 * t88224 + 0.11651625e2_f64 * t88226 - 0.22076e0_f64 * t88229 + 0.66228e0_f64 * t88232 + 0.72462e1_f64 * t88085 + 0.181155e1_f64 * t88093 - 0.89459259259259259259e0_f64 * t88104 - 0.301925e0_f64 * t88108 + 0.40256666666666666666e1_f64 * t88114 - 0.72462e1_f64 * t88122 - 0.60384999999999999999e0_f64 * t88130 - 0.132456e1_f64 * t77804;
    (t88220, t88222, t88224, t88226, t88229, t88232, t88242)
}
