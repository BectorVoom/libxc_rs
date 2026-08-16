//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 837/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk837(t1238: f64, t2121: f64, t2155: f64, t498: f64, t7283: f64, t7351: f64, t8868: f64, t8872: f64, t8883: f64, t8888: f64, t8898: f64, t2157: f64) -> (f64, f64) {
    let t8900 = 0.16449340668482264365e-1_f64 * t2121 * t8868 - 0.16449340668482264365e-1_f64 * t7283 * t8872 + t8883 * t498 - 2.0_f64 * t7351 * t2155 + 2.0_f64 * t1238 * t8888 - t1238 * t8898;
    let t8904 = t2157 * t2157;
    (t8900, t8904)
}
