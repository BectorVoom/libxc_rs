//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1219/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1219(t871: f64, t8710: f64, t27837: f64, t27840: f64, t27844: f64, t27856: f64, t27858: f64, t27860: f64, t32300: f64, t32302: f64, t32304: f64, t739: f64) -> (f64, f64) {
    let t32305 = t8710 * t871;
    let t32307 = 63.0_f64 / 512.0_f64 * t27837;
    let t32308 = 385.0_f64 / 16384.0_f64 * t27840;
    let t32309 = 147.0_f64 / 1048576.0_f64 * t27844;
    let t32310 = 49.0_f64 / 1048576.0_f64 * t27856;
    let t32311 = 385.0_f64 / 49152.0_f64 * t27858;
    let t32312 = 21.0_f64 / 512.0_f64 * t27860;
    let t32313 = t32300 - t32302 + t32304 + t32305 / 2.0_f64 + t32307 - t32308 + t32309 - t32310 + t32311 - t32312;
    let t32314 = t739 * t32313;
    (t32313, t32314)
}
