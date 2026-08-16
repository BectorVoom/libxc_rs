//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1235/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1235(t12570: f64, t792: f64, t3262: f64, t3276: f64, t3275: f64, t3582: f64, t40705: f64, t11519: f64, t40282: f64, t10918: f64, t12391: f64, t42846: f64, t795: f64) -> (f64, f64, f64, f64, f64) {
    let t43729 = t12570 * t792;
    let t43732 = 15.0_f64 / 16.0_f64 * t3262 * t3276 * t43729;
    let t43735 = 5.0_f64 / 8.0_f64 * t3275 * t40705 * t3582;
    let t43739 = 15.0_f64 / 8.0_f64 * t40282 * t11519;
    let t43742 = 3.0_f64 / 2.0_f64 * t3262 * t10918 * t12391;
    let t43744 = t42846 * t795;
    (t43732, t43735, t43739, t43742, t43744)
}
