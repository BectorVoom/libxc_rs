//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1621/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1621(t61247: f64, t61282: f64, t50852: f64, t50856: f64, t61294: f64, t61296: f64, t39989: f64, t40067: f64, t40072: f64, t40167: f64, t40171: f64, t62276: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t87666 = 0.65061487801810439052e-1_f64 * t61247;
    let t87667 = 0.14649157844805236043e-2_f64 * t61282;
    let t87668 = 0.2077903092681775651e3_f64 * t50852;
    let t87669 = 0.22787578869697033845e-2_f64 * t50856;
    let t87670 = 0.35089341735807877242e1_f64 * t61294;
    let t87671 = 0.10389515463408878255e3_f64 * t61296;
    let t87672 = t87666 + t87667 - t39989 - t87668 - t87669 - t87670 - t87671 + t40067 - t40072 + t40167 - t40171;
    let t87673 = 0.70178683471615754484e1_f64 * t62276;
    (t87666, t87667, t87668, t87669, t87670, t87671, t87672, t87673)
}
