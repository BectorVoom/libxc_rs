//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2306/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2306(t57965: f64, t40722: f64, t40733: f64, t57992: f64, t185: f64, t67060: f64, t707: f64, t21066: f64, t2752: f64, t145: f64, t67083: f64, t20767: f64, t751: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t67137 = 36.0_f64 * t57965;
    let t67141 = 0.56968947174242584612e-3_f64 * t40722;
    let t67146 = 0.35089341735807877242e1_f64 * t40733;
    let t67147 = 12.0_f64 * t57992;
    let t67153 = 4.0_f64 * t707 * t185 * t67060;
    let t67154 = t21066 * t2752;
    let t67158 = t145 * t67083 * t185;
    let t67159 = t20767 * t751;
    (t67137, t67141, t67146, t67147, t67153, t67154, t67158, t67159)
}
