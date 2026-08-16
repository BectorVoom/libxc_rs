//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 560/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk560(t428: f64, t7883: f64, t1711: f64, t6: f64, t64: f64, t1300: f64, t1617: f64, t1620: f64, t1683: f64, t1687: f64, t1698: f64, t1701: f64, t1702: f64, t1712: f64, t2035: f64, t3065: f64, t399: f64, t403: f64, t7833: f64, t7838: f64, t7840: f64, t7845: f64, t7848: f64, t7852: f64, t7854: f64, t7860: f64, t7861: f64, t7867: f64, t7868: f64, t7877: f64, t7879: f64) -> (f64, f64) {
    let t7884 = t7883 * t428;
    let t7888 = t1711 * t6;
    let t7889 = t64 * t7888;
    let t7894 = -0.20676097475611486194e-3_f64 * t1617 * t7833 * t1620 - 0.82704389902445944777e-3_f64 * t7838 * t7840 + 0.41352194951222972388e-3_f64 * t7845 * t7848 - 0.7112856777411015585e-1_f64 * t7852 * t7854 + 0.48082059875423759229e-5_f64 * t7860 * t7861 - 0.22524046461801549353e0_f64 * t403 * t1683 - 0.42160609613301514757e-3_f64 * t7867 * t2035 * t7868 + 0.35564283887055077925e-1_f64 * t1687 * t399 + 0.84321219226603029514e-3_f64 * t403 * t1698 + 0.139529405678626752e0_f64 * t7877 * t3065 * t7879 + 0.11262023230900774676e0_f64 * t1300 * t1701 * t7884 + 0.35564283887055077925e-1_f64 * t7889 * t1701 * t1702 * t1712;
    (t7889, t7894)
}
