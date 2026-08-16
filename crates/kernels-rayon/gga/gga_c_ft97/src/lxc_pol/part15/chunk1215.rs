//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1215/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1215(t18826: f64, t21373: f64, t4068: f64, t4977: f64, t80087: f64, t80089: f64, t80091: f64, t80096: f64, t88728: f64, t88732: f64, t88737: f64, t88758: f64, t88766: f64, t89947: f64, t89950: f64, t89981: f64) -> f64 {
    let t91307 = 0.86658499999999999998e0_f64 * t88758 + 0.19257444444444444444e1_f64 * t88766 - 0.42794320987654320987e0_f64 * t88728 - 0.14443083333333333333e0_f64 * t88732 - 0.34663399999999999999e1_f64 * t88737 - 0.469508e0_f64 * t4068 * t21373 + 0.1056393e1_f64 * t18826 * t4977 - 0.77029777777777777776e0_f64 * t80087 + 0.11554466666666666666e1_f64 * t80089 + 0.21397160493827160493e0_f64 * t80091 + 0.19257444444444444444e0_f64 * t80096 - 0.44016375e0_f64 * t89947 - 0.352131e0_f64 * t89950 + 0.234754e0_f64 * t89981;
    t91307
}
