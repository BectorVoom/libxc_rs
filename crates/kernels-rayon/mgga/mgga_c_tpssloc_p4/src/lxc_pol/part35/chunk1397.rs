//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1397/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1397(t106951: f64, t1268: f64, t1458: f64, t5449: f64, t1873: f64, t19451: f64, t7467: f64, t1983: f64, t2019: f64, t74014: f64, t1390: f64, t2018: f64, t20356: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t106953 = 2.0_f64 * t1268 * t106951;
    let t106956 = t5449 * t1458;
    let t106958 = 6.0_f64 * t106956 * t1873;
    let t106960 = 6.0_f64 * t19451 * t7467;
    let t106964 = t1983 * t2019 * t74014;
    let t106968 = 6.0_f64 * t1983 * t20356 * t2018 * t1390;
    (t106953, t106956, t106958, t106960, t106964, t106968)
}
