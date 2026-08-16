//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 310/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk310(t779: f64, t937: f64, t2272: f64, t286: f64, t708: f64, t1687: f64, t2277: f64, t1232: f64, t1692: f64, t1685: f64, t2276: f64, t716: f64, t926: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t2509 = t779 * t937;
    let t2513 = t2272 * t286 * t708;
    let t2515 = t2277 * t1687;
    let t2517 = t1692 * t1232;
    let t2518 = t2276 * t1685;
    let t2519 = t2518 * pi;
    let t2520 = t2517 * t2519;
    let t2522 = t926 * t716;
    (t2509, t2513, t2515, t2518, t2519, t2520, t2522)
}
