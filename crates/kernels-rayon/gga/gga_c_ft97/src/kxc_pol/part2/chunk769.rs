//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 769/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk769(t11484: f64, t11546: f64, t11591: f64, t11822: f64, t11876: f64, t11910: f64, t11997: f64, t12055: f64, t103: f64, t11801: f64, t108: f64, t11420: f64, t11424: f64, t11427: f64, t11538: f64, t11542: f64, t11816: f64, t11838: f64, t11961: f64, t2976: f64, t497: f64, t88: f64) -> f64 {
    let t12058 = t11484 + t11546 + t11591 + t11822 + t11876 + t11910 + t11997 + t12055;
    let t12062 = t11801 * t103;
    let t12067 = -t108 * t11420 - 2.0_f64 * t108 * t11424 - t108 * t11427 - t12058 * t88 - 2.0_f64 * t2976 * t497 - 2.0_f64 * t11538 - 4.0_f64 * t11542 - 2.0_f64 * t11816 - 4.0_f64 * t11838 - 2.0_f64 * t11961 + 2.0_f64 * t12062;
    t12067
}
