//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3269/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3269(t74130: f64, t74132: f64, t48313: f64, t47110: f64, t189: f64, t512: f64, t85970: f64, t22789: f64, t749: f64, t47119: f64, t40067: f64, t40072: f64, t47109: f64, t47113: f64, t47116: f64, t47118: f64, t47122: f64, t47124: f64, t48312: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t85979 = 0.35089341735807877242e1_f64 * t74130;
    let t85980 = 24.0_f64 * t74132;
    let t85981 = 0.65061487801810439052e-1_f64 * t48313;
    let t85982 = 0.35089341735807877242e1_f64 * t47110;
    let t85984 = t512 * t85970 * t189;
    let t85986 = t512 * t22789 * t749;
    let t85987 = 0.32530743900905219526e-1_f64 * t47119;
    let t85988 = t85979 - t85980 + t48312 - t85981 + t40067 - t40072 - t47109 - t85982 + t85984 + t85986 + t47113 + t47116 - t47118 - t85987 + t47122 + t47124;
    (t85979, t85980, t85981, t85982, t85984, t85986, t85987, t85988)
}
