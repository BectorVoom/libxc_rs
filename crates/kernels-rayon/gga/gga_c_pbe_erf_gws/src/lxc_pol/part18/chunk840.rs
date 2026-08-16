//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 840/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk840(t8122: f64, t8124: f64, t485: f64, t974: f64, t156: f64, t2881: f64, t496: f64, t2874: f64, t395: f64, t1508: f64, t971: f64, t1251: f64) -> (f64, f64, f64, f64, f64) {
    let t8126 = 0.587616e1_f64 * t8122 * t8124;
    let t8135 = t485 * t974;
    let t8137 = 0.19486833333333333333e1_f64 * t8135 * t8124;
    let t8139 = t496 * t156 * t2881;
    let t8140 = t485 * t2874;
    let t8142 = 0.97434166666666666666e0_f64 * t8140 * t395;
    let t8143 = t1508 * t971;
    let t8144 = t8143 * t1251;
    (t8126, t8137, t8139, t8142, t8144)
}
