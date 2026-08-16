//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 844/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk844(t11795: f64, t11797: f64, t11800: f64, t11803: f64, t11806: f64, t11811: f64, t11813: f64, t11815: f64, t11817: f64, t11820: f64, t201: f64, t219: f64) -> f64 {
    let t11825 = 1.0_f64 * t201 * (-0.21099166666666666667e1_f64 * t11795 + 0.202552e2_f64 * t11797 - 0.75019259259259259258e1_f64 * t11800 + 0.6564185185185185185e1_f64 * t11803 + 0.31003950617283950618e1_f64 * t11806 + 0.68258333333333333335e-1_f64 * t11811 - 0.10921333333333333333e1_f64 * t11813 + 0.12134814814814814815e1_f64 * t11815 + 0.10617962962962962963e1_f64 * t11817 + 0.13388493827160493828e1_f64 * t11820) * t219;
    t11825
}
