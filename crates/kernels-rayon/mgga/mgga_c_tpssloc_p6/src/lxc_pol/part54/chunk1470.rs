//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1470/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1470(t120067: f64, t121195: f64, t121197: f64, t121199: f64, t121201: f64, t121203: f64, t121204: f64, t121211: f64, t121224: f64, t123194: f64, t26559: f64, t31055: f64, t31057: f64, t31060: f64, t31832: f64, t7943: f64) -> f64 {
    let t124951 = 2.0_f64 * t123194 * t26559 - t31832 * t7943 - t120067 - t121195 - t121197 - t121199 - t121201 + t121203 - t121204 + t121211 - t121224 - t31055 - t31057 - t31060;
    t124951
}
