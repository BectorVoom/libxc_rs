//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 768/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk768(t1853: f64, t920: f64, t12045: f64, t1909: f64, t3114: f64, t8506: f64, t11593: f64, t11999: f64, t12002: f64, t12005: f64, t12009: f64, t12013: f64, t12017: f64, t12022: f64, t12027: f64, t12030: f64, t12035: f64, t12038: f64, t12042: f64, t1901: f64, t8567: f64) -> f64 {
    let t12046 = t920 * t1853;
    let t12047 = t12045 * t12046;
    let t12048 = t1909 * t12047;
    let t12051 = t8506 * t3114;
    let t12055 = -t11999 + 22.0_f64 / 27.0_f64 * t12002 + 2.0_f64 / 27.0_f64 * t1901 * t12005 + t1901 * t12009 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t1901 * t12013 + 2.0_f64 / 9.0_f64 * t1901 * t12017 + 4.0_f64 / 9.0_f64 * t1901 * t12022 + 2.0_f64 / 9.0_f64 * t1901 * t12027 + 2.0_f64 / 9.0_f64 * t1901 * t12030 + 2.0_f64 / 9.0_f64 * t1901 * t12035 + 2.0_f64 / 9.0_f64 * t1901 * t12038 - 8.0_f64 / 27.0_f64 * t11593 * t12042 - 2.0_f64 / 9.0_f64 * t1901 * t12048 + 2.0_f64 / 9.0_f64 * t1901 * t12051 + 2.0_f64 / 27.0_f64 * t8567;
    t12055
}
