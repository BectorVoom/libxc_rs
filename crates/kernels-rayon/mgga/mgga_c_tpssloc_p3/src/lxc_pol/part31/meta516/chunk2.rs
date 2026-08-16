//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1714/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1714(t5: f64, t109: f64, t28941: f64, t112: f64, t23912: f64, t26127: f64, t28012: f64, t28014: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t110 = 1.0_f64 < t109;
    let t28942 = piecewise3(t8, 0.0_f64, t28941);
    let t28943 = t28942 * t112;
    let t28951 = piecewise3(t110, 0.0_f64, t23912 + 4.0_f64 / 3.0_f64 * t26127 + t28012 / 2.0_f64 - t28014 / 4.0_f64);
    (t28942, t28943, t28951)
}
