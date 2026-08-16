//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1888/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1888(t21089: f64, t2929: f64, t951: f64, t959: f64, t10523: f64, t2932: f64, t1589: f64, t17934: f64, t10629: f64, t10632: f64, t4483: f64, t5808: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21091 = t2929 * t21089 * t951;
    let t21093 = 0.35089341735807877242e1_f64 * t959 * t21091;
    let t21094 = t10523 * t21089;
    let t21095 = t21094 * t2932;
    let t21097 = 0.10389515463408878255e3_f64 * t959 * t21095;
    let t21099 = 0.17544670867903938621e1_f64 * t17934 * t1589;
    let t21100 = t10629 * t21089;
    let t21101 = t21100 * t10632;
    let t21103 = 0.10254018858216406658e4_f64 * t959 * t21101;
    let t21105 = 0.17544670867903938621e1_f64 * t4483 * t5808;
    (t21091, t21093, t21094, t21095, t21097, t21099, t21100, t21101, t21103, t21105)
}
