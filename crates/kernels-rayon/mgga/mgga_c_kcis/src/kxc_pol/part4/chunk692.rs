//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 692/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk692(t1396: f64, t3954: f64, t1468: f64, t1464: f64, t1362: f64, t506: f64, t486: f64) -> (f64, f64, f64, f64, f64) {
    let t3955 = t1396 * t3954;
    let t3956 = t1468 * t3955;
    let t3957 = t1464 * t3956;
    let t3960 = 1.0_f64 / t1362 / t506;
    let t3961 = t486 * t3960;
    (t3955, t3956, t3957, t3960, t3961)
}
