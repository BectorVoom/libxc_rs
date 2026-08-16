//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 459/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk459(t1664: f64, t356: f64, t552: f64, t848: f64, t833: f64, t108: f64, t1539: f64) -> (f64, f64, f64, f64) {
    let t5002 = t1664 * t356;
    let t5005 = t552 * t848;
    let t5008 = t552 * t833;
    let t5011 = t1539 * t108;
    (t5002, t5005, t5008, t5011)
}
