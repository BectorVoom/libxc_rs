//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1168/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1168(t2121: f64, t3427: f64, t8891: f64, t1170: f64, t32469: f64, t24574: f64, t32462: f64, t32459: f64, t477: f64, t7348: f64, t32551: f64, t3640: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t118142 = 0.36554090374405031922e-2_f64 * t2121 * t3427 * t8891;
    let t118157 = t2121 * t1170 * t32469;
    let t118162 = t24574 * t32462;
    let t118173 = t24574 * t32459;
    let t118175 = t477 * t7348;
    let t118229 = t32551 * t3640;
    (t118142, t118157, t118162, t118173, t118175, t118229)
}
