//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 801/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk801(t3722: f64, t954: f64, t2508: f64, t13861: f64, t2580: f64, t12255: f64, t948: f64, t13180: f64, t13187: f64, t13189: f64, t13193: f64, t13197: f64, t13198: f64, t13204: f64, t13215: f64, t13216: f64, t13220: f64) -> (f64, f64, f64, f64) {
    let t13918 = t954 * t3722;
    let t13919 = t2508 * t13918;
    let t13921 = t2580 * t13861;
    let t13922 = t2508 * t13921;
    let t13924 = t12255 * t948;
    let t13925 = t2508 * t13924;
    let t13930 = 0.76905262301422242837e-2_f64 * t13919 + 0.15381052460284448567e-1_f64 * t13922 - 0.23071578690426672851e-1_f64 * t13925 + t13216 + t13220 - t13197 + t13198 - t13215 - 0.23071578690426672851e-1_f64 * t13180 - t13187 + 0.76905262301422242837e-2_f64 * t13189 + t13193 + 0.15381052460284448567e-1_f64 * t13204;
    (t13918, t13921, t13924, t13930)
}
