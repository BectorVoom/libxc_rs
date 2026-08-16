//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 927/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk927(t1533: f64, t2858: f64, t5770: f64, t1504: f64, t2893: f64, t501: f64, t156: f64, t4: f64, t481: f64, t2897: f64, t978: f64, t485: f64, t974: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8112 = t2858 * t1533;
    let t8117 = 0.48717083333333333333e0_f64 * t5770;
    let t8118 = t2893 * t1504;
    let t8122 = t501 * t2893;
    let t8124 = t4 * t156 * t481;
    let t8126 = 0.587616e1_f64 * t8122 * t8124;
    let t8127 = t2897 * t481;
    let t8131 = t978 * t1533;
    let t8135 = t485 * t974;
    (t8112, t8117, t8118, t8124, t8126, t8127, t8131, t8135)
}
