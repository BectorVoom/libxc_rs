//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 104/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk104(t247: f64, t250: f64, t369: f64, t374: f64, t179: f64) -> (f64, f64) {
    let t416 = -0.86308333333333333334e0_f64 * t247 - 0.301925e0_f64 * t250 - 0.5501625e-1_f64 * t369 - 0.82785e-1_f64 * t374;
    let t417 = 1.0_f64 / t179;
    (t416, t417)
}
