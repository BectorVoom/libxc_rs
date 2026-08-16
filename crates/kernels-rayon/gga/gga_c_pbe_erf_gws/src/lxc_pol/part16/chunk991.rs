//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 991/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk991(t2118: f64, t3106: f64, t3074: f64, t745: f64, t857: f64, t858: f64, t856: f64, t3178: f64, t810: f64, t337: f64, t6560: f64, t2146: f64) -> (f64, f64, f64, f64) {
    let t8860 = t2118 * t3106;
    let t8861 = t3074 * t8860;
    let t8863 = t857 * t858 * t745;
    let t8864 = t856 * t8863;
    let t8866 = t8861 * t8864 / 32.0_f64;
    let t8867 = t3178 * t810;
    let t8868 = t337 * t8867;
    let t8869 = t6560 * t8868;
    let t8871 = t2146 * t8869 / 8.0_f64;
    (t8860, t8866, t8867, t8871)
}
