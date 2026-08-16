//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1096/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1096(t2243: f64, t5870: f64, t303: f64, t1458: f64, t8175: f64, t3964: f64, t6140: f64, t1385: f64, t1650: f64, t27356: f64, t5709: f64, t27453: f64, t5654: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28473 = t5870 * t2243;
    let t28474 = t303 * t28473;
    let t28476 = t1458 * t8175;
    let t28477 = t303 * t28476;
    let t28480 = t3964 * t6140;
    let t28483 = t1650 * t1385;
    let t28484 = t27356 * t28483;
    let t28485 = t5709 * t28484;
    let t28488 = t27453 * t5654;
    (t28473, t28474, t28476, t28477, t28480, t28484, t28485, t28488)
}
