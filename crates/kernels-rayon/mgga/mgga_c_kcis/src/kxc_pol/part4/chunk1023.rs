//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1023/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1023(t530: f64, t64: f64, t555: f64, t491: f64, t1505: f64, t4182: f64, t1502: f64, t4188: f64, t1504: f64, t561: f64, t1507: f64, t456: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12319 = t64 * t530;
    let t12321 = 1.0_f64 / t555 / t12319;
    let t12322 = t491 * t12321;
    let t12335 = t4182 * t1505;
    let t12338 = t1502 * t4188;
    let t12343 = t1504 * t1504;
    let t12344 = 1.0_f64 / t12343;
    let t12345 = t561 * t12344;
    let t12361 = t1507 * t456;
    (t12321, t12322, t12335, t12338, t12345, t12361)
}
