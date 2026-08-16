//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1022/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1022(t10710: f64, t162: f64, t3566: f64, t10701: f64, t10566: f64, t10568: f64, t10686: f64, t10692: f64, t14119: f64, t14123: f64, t14129: f64, t14130: f64, t14137: f64, t14138: f64, t14139: f64, t14140: f64, t14141: f64, t1692: f64, t2439: f64, t2440: f64, t3548: f64, t3552: f64, t3683: f64, t4701: f64, t750: f64, t8117: f64, t8126: f64, t821: f64) -> (f64, f64, f64) {
    let t14142 = t10710 * t162;
    let t14144 = 24.0_f64 * t14142 * t3566;
    let t14145 = 0.23392894490538584828e1_f64 * t10701;
    let t14146 = 2.0_f64 * t14123 * t1692 * t821 - 3.0_f64 * t14130 * t2439 * t750 + 3.0_f64 * t2439 * t2440 * t4701 + 12.0_f64 * t3548 * t3552 * t3683 + t10566 + t10568 - t10686 + t10692 + t14119 + t14129 - t14137 - t14138 + t14139 - t14140 + t14141 + t14144 + t14145 - t8117 - t8126;
    (t14144, t14145, t14146)
}
