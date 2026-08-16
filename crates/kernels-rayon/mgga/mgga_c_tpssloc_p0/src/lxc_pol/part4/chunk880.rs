//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 880/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk880(t761: f64, t9919: f64, t2531: f64, t2535: f64, t32: f64, t717: f64, t2617: f64, t2629: f64, t813: f64, t236: f64, t232: f64, t2632: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9921 = 0.35089341735807877242e1_f64 * t761 * t9919;
    let t9922 = t2531 * t2535;
    let t9929 = t32 * t717;
    let t9967 = t2617 * t2629;
    let t9970 = t813 * t813;
    let t9971 = 1.0_f64 / t9970;
    let t9972 = t9971 * t236;
    let t9975 = t2632 * t232;
    (t9921, t9922, t9929, t9967, t9971, t9972, t9975)
}
