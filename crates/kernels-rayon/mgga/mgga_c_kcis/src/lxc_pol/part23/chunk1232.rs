//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1232/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1232(t12234: f64, t531: f64, t1650: f64, t3715: f64, t5709: f64, t1394: f64, t16700: f64, t27387: f64, t28519: f64, t4142: f64, t15919: f64, t28503: f64) -> (f64, f64, f64, f64, f64) {
    let t98084 = t12234 * t531;
    let t98087 = t5709 * t98084 * t1650 * t3715;
    let t98102 = t1394 * t27387 * t16700;
    let t98104 = t4142 * t28519;
    let t98105 = 0.22109259259259259258e-2_f64 * t98104;
    let t98107 = t1394 * t28503 * t15919;
    (t98087, t98102, t98104, t98105, t98107)
}
