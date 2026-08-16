//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1046/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1046(t1005: f64, t4557: f64, t1487: f64, t336: f64, t1319: f64, t3570: f64, t1137: f64, t5161: f64, t3621: f64, t5165: f64, t3382: f64, t4295: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18121 = t1005 * t4557;
    let t18129 = t336 * t1487;
    let t18139 = t3570 * t1319;
    let t18141 = t1137 * t5161;
    let t18147 = t3621 * t5165;
    let t18153 = t3382 * t4295;
    (t18121, t18129, t18139, t18141, t18147, t18153)
}
