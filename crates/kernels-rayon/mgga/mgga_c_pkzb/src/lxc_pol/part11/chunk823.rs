//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 823/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk823(t1536: f64, t3401: f64, t5149: f64, t1020: f64, t1535: f64, t2536: f64, t2718: f64, t3396: f64, t5091: f64, t5130: f64, t5139: f64, t5141: f64, t5148: f64, t637: f64, t7015: f64, t7017: f64, t7019: f64, t7022: f64, t7201: f64, t8769: f64, t8772: f64, t8773: f64, t8774: f64, t8776: f64, t8778: f64, t8779: f64) -> (f64, f64) {
    let t8783 = t1536 * t3401;
    let t8789 = 0.11696447245269292414e1_f64 * t5149;
    let t8793 = 6.0_f64 * t1020 * t1535 * t7201 + 3.0_f64 * t1535 * t1536 * t3396 + 2.0_f64 * t2536 * t637 * t8779 + 6.0_f64 * t2718 * t8783 + t5091 - t5130 - t5139 + t5141 - t5148 - t7015 - t7017 + t7019 + t7022 - t8769 - t8772 - t8773 - t8774 + t8776 + t8778 + t8789;
    (t8789, t8793)
}
