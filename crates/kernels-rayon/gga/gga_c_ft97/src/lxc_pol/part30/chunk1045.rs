//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1045/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1045(t24437: f64, t2574: f64, t27850: f64, t6119: f64, t27819: f64, t27820: f64, t33319: f64, t150288: f64, t150291: f64, t150295: f64, t150298: f64, t150302: f64, t150304: f64, t150308: f64, t150915: f64, t150918: f64, t150922: f64, t150927: f64, t150931: f64, t150935: f64, t150939: f64) -> (f64, f64, f64) {
    let t150943 = t24437 * t2574 * t6119 * t27850;
    let t150946 = t27819 * t2574 * t33319 * t27820;
    let t150948 = 3.0_f64 / 2.0_f64 * t150288 + t150291 / 3.0_f64 - 2.0_f64 * t150295 + 2.0_f64 * t150298 + 4.0_f64 * t150302 - 4.0_f64 / 9.0_f64 * t150304 + 3.0_f64 * t150308 - t150915 - 6.0_f64 * t150918 + 2.0_f64 * t150922 + t150927 + t150931 + t150935 / 4.0_f64 + 3.0_f64 / 2.0_f64 * t150939 - t150943 + 3.0_f64 / 4.0_f64 * t150946;
    (t150943, t150946, t150948)
}
