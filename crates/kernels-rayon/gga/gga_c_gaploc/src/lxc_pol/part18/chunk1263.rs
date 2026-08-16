//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1263/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1263(t10913: f64, t10916: f64, t1980: f64, t14630: f64, t3025: f64, t948: f64, t20157: f64, t2085: f64, t320: f64, t32613: f64, t1969: f64, t3294: f64, t5746: f64, t8604: f64) -> (f64, f64, f64, f64) {
    let t32853 = 0.42900587942220512002e1_f64 * t1980 * t10913 * t10916;
    let t32856 = 0.23833659967900284447e0_f64 * t3025 * t14630 * t948;
    let t32860 = 0.27606906686822939768e2_f64 * t320 * t2085 * t20157 * t32613;
    let t32866 = 0.12269736305254639897e2_f64 * t320 * t5746 * t20157 * t8604 * t3294 * t1969;
    (t32853, t32856, t32860, t32866)
}
