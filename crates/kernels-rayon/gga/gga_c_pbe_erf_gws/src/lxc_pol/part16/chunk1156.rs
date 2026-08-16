//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1156/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1156(t1205: f64, t3306: f64, t2409: f64, t3067: f64, t4216: f64, t8734: f64, t1105: f64, t4110: f64, t2376: f64, t14185: f64, t3060: f64, t9283: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14943 = t1205 * t3306;
    let t14945 = t2409 * t3067 * t14943;
    let t14949 = t2409 * t8734 * t4216;
    let t14952 = t4110 * t1105;
    let t14954 = t2409 * t2376 * t14952;
    let t14958 = t14185 * t3060;
    let t14959 = t9283 * t14958;
    (t14943, t14945, t14949, t14952, t14954, t14958, t14959)
}
