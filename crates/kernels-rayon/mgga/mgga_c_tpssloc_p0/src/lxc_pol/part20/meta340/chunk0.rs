//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1639/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1639(t1372: f64, t3752: f64, t1376: f64, t68: f64, t1385: f64, t3888: f64, t3911: f64, t3887: f64, t225: f64, t3753: f64, t3880: f64, t1323: f64, t3879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12016 = t3752 * t1372;
    let t12019 = t1376 * t1376;
    let t12020 = 1.0_f64 / t12019;
    let t12021 = t68 * t12020;
    let t12022 = t3888 * t1385;
    let t12023 = t12021 * t12022;
    let t12026 = t1385 * t3911;
    let t12027 = t3887 * t12026;
    let t12030 = t3753 * t225;
    let t12033 = t3880 * t225;
    let t12036 = t1323 * t3879;
    (t12016, t12019, t12020, t12021, t12022, t12023, t12026, t12027, t12030, t12033, t12036)
}
