//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1234/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1234(t1969: f64, t20157: f64, t320: f64, t3294: f64, t5746: f64, t8604: f64, t22826: f64, t3009: f64, t590: f64, t7068: f64, t23516: f64, t32616: f64) -> (f64, f64, f64) {
    let t32866 = 0.12269736305254639897e2_f64 * t320 * t5746 * t20157 * t8604 * t3294 * t1969;
    let t32870 = 0.30674340763136599742e1_f64 * t22826 * t3009 * t7068 * t590;
    let t32872 = 0.51123901271894332902e1_f64 * t23516 * t32616;
    (t32866, t32870, t32872)
}
