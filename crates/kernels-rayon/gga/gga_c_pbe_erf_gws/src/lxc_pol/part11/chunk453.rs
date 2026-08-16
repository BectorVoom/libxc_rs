//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 453/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk453(t331: f64, t589: f64, t597: f64, t995: f64, t1036: f64, t1630: f64, t639: f64, t1009: f64, t1651: f64, t587: f64, t1061: f64, t719: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2620 = t331 * t589;
    let t2635 = t597 * t995;
    let t2640 = t1630 * t1036;
    let t2641 = t639 * t2640;
    let t2643 = t1651 * t1009;
    let t2644 = t587 * t2643;
    let t2650 = t1061 * t719;
    (t2620, t2635, t2640, t2641, t2643, t2644, t2650)
}
