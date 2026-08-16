//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1150/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1150(t1268: f64, t28017: f64, t1458: f64, t24999: f64, t27993: f64, t27996: f64, t28001: f64, t28004: f64, t28006: f64, t28009: f64, t28011: f64, t5493: f64, t6517: f64) -> f64 {
    let t28019 = 2.0_f64 * t1268 * t28017;
    let t28020 = 4.0_f64 * t1458 * t24999 + 2.0_f64 * t5493 * t6517 + t27993 + 2.0_f64 * t27996 + t28001 + t28004 + t28006 + t28009 + t28011 + t28019;
    t28020
}
