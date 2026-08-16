//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1103/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1103(t19733: f64, t2382: f64, t833: f64, t4867: f64, t823: f64, t825: f64, t2112: f64, t328: f64, t331: f64, t745: f64, t2416: f64, t810: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19735 = t2382 * t19733 * t833;
    let t19737 = t823 * t4867;
    let t19738 = t19737 * t825;
    let t19744 = t2112 * t328;
    let t19745 = t19744 * t331;
    let t19750 = t745 * t328;
    let t19751 = t19750 * t331;
    let t19756 = t2416 * t810;
    (t19735, t19737, t19738, t19744, t19745, t19750, t19751, t19756)
}
