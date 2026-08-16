//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1200/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1200(t10657: f64, t64: f64, t3427: f64, t90: f64, t27837: f64, t27840: f64, t27844: f64, t27856: f64, t27858: f64, t27860: f64, t10691: f64, t21665: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32302 = 8.0_f64 / 3.0_f64 * t10657 * t64;
    let t32304 = 4.0_f64 / 3.0_f64 * t3427 * t90;
    let t32307 = 63.0_f64 / 512.0_f64 * t27837;
    let t32308 = 385.0_f64 / 16384.0_f64 * t27840;
    let t32309 = 147.0_f64 / 1048576.0_f64 * t27844;
    let t32310 = 49.0_f64 / 1048576.0_f64 * t27856;
    let t32311 = 385.0_f64 / 49152.0_f64 * t27858;
    let t32312 = 21.0_f64 / 512.0_f64 * t27860;
    let t32328 = t21665 * t10691;
    (t32302, t32304, t32307, t32308, t32309, t32310, t32311, t32312, t32328)
}
