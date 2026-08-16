//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1009/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1009(t3626: f64, t751: f64, t481: f64, t981: f64, t5651: f64, t3685: f64, t475: f64, t142: f64, t3644: f64, t525: f64, t2919: f64, t524: f64) -> (f64, f64, f64, f64, f64) {
    let t11290 = t751 * t3626;
    let t11292 = t981 * t481;
    let t11293 = t5651 * t11292;
    let t11296 = t475 * t3685;
    let t11299 = t142 * t3644;
    let t11300 = t525 * t11299;
    let t11303 = t524 * t2919;
    (t11290, t11293, t11296, t11300, t11303)
}
