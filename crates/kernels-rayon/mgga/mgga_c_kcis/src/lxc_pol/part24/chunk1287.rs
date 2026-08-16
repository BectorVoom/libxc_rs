//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1287/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1287(t2811: f64, t2844: f64, t1008: f64, t6272: f64, t4939: f64, t27820: f64, t95915: f64, t95921: f64, t28987: f64, t93435: f64, t26685: f64, t27806: f64, t81752: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t101063 = t2811 * t2844;
    let t101064 = t6272 * t1008;
    let t101066 = t4939 * t101063 * t101064;
    let t101072 = t95921 * t95915 * t27820;
    let t101077 = t93435 * t28987;
    let t101078 = t26685 * t101077;
    let t101084 = t27806 * t81752;
    (t101064, t101066, t101072, t101077, t101078, t101084)
}
