//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 901/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk901<F: Float>(t1992: F, t7585: F, t7586: F, t8906: F, t1983: F, t8402: F, t30105: F, t8897: F, t30268: F, t8783: F, t31254: F, t1479: F, t429: F, t1980: F, t7476: F, t31262: F) -> (F, F, F, F, F, F, F, F) {
    let t35479 = t7585 * t7586 * t1992 * t8906;
    let t35484 = t7585 * t7586 * t1983 * t8402;
    let t35486 = t30105 * t8897;
    let t35496 = t30268 * t8783;
    let t35499 = 0.85748036236139473944e-3 * t31254;
    let t35500 = t429 * t1479;
    let t35502 = t1980 * t7476 * t35500;
    let t35506 = 0.26147916666666666666e0 * t31262;
    (t35479, t35484, t35486, t35496, t35499, t35500, t35502, t35506)
}
