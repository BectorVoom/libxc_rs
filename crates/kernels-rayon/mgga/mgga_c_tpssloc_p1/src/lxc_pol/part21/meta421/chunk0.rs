//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1940/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1940(t1751: f64, t3493: f64, t1246: f64, t3507: f64, t3625: f64, t1932: f64, t475: f64, t1755: f64, t1720: f64, t3030: f64, t3609: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15015 = t1751 * t3493;
    let t15016 = t15015 * t1246;
    let t15018 = t1751 * t3507;
    let t15019 = t15018 * t3625;
    let t15022 = t1932 * t3493 * t475;
    let t15023 = t1755 * t15022;
    let t15026 = t1720 * t3030;
    let t15027 = t15026 * t3609;
    (t15016, t15018, t15019, t15022, t15023, t15026, t15027)
}
