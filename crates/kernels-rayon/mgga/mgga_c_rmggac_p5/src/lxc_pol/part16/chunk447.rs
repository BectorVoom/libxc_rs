//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 447/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk447(t4698: f64, t4700: f64, t1664: f64, t356: f64, t108: f64, t1539: f64) -> (f64, f64, f64, f64) {
    let t4997 = 1584.0_f64 * t4698;
    let t4998 = 1872.0_f64 * t4700;
    let t5002 = t1664 * t356;
    let t5011 = t1539 * t108;
    (t4997, t4998, t5002, t5011)
}
