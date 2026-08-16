//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 810/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk810(t1900: f64, t227: f64, t5737: f64, t5802: f64, t1954: f64, t1972: f64, t721: f64, t730: f64, t5519: f64, t5522: f64, t5525: f64, t5539: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5804 = 1.0_f64 / t1900 / t227;
    let t5805 = t5737 * t5804;
    let t5807 = 0.51726012919273400301e3_f64 * t5802 * t5805;
    let t5809 = t1954 * t721 * t1972;
    let t5811 = 0.35089341735807877242e1_f64 * t730 * t5809;
    let t5812 = 0.53272592592592592592e-1_f64 * t5519;
    let t5816 = -t5812 + 0.68493333333333333332e-1_f64 * t5522 - 0.51369999999999999999e-1_f64 * t5525 + 0.5137e-1_f64 * t5539;
    (t5804, t5805, t5807, t5809, t5811, t5812, t5816)
}
