//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 799/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk799(t8643: f64, t960: f64, t8649: f64, t8652: f64, t965: f64, t1850: f64, t7715: f64, t696: f64, t7718: f64, t5136: f64, t4811: f64, t8948: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23251 = t960 * t8643;
    let t23253 = t960 * t8649;
    let t23255 = t965 * t8652;
    let t23259 = t1850 * t7715;
    let t23261 = t696 * t7718;
    let t23263 = t5136 * t7718;
    let t23286 = t4811 * t8948;
    (t23251, t23253, t23255, t23259, t23261, t23263, t23286)
}
