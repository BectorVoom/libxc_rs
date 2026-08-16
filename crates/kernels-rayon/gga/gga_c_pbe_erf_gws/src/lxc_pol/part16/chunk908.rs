//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 908/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk908(t1621: f64, t7920: f64, t1620: f64, t256: f64, t7867: f64, t7870: f64, t7873: f64, t7876: f64, t7880: f64, t7884: f64, t7887: f64, t7890: f64, t7894: f64, t7898: f64, t7903: f64, t7905: f64, t7910: f64, t7915: f64, t7917: f64, t7919: f64) -> (f64, f64) {
    let t7921 = t1621 * t7920;
    let t7923 = 4.0_f64 / 15.0_f64 * t1620 * t7921;
    let t7924 = -t7867 + t7870 - t7873 - t7876 + t7880 - t7884 - t7887 + t7890 + t7894 + t7898 + t7903 - t7905 + t7910 * t256 / 3.0_f64 - t7915 - t7917 + t7919 - t7923;
    (t7923, t7924)
}
