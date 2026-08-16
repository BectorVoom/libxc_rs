//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1190/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1190(t2191: f64, t3065: f64, t2159: f64, t14028: f64, t2308: f64, t14063: f64, t6257: f64, t14092: f64, t6569: f64, t14022: f64, t828: f64, t2123: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51309 = t3065 * t2191;
    let t51312 = t3065 * t2159;
    let t51315 = t14028 * t2308;
    let t51317 = t14063 * t6257;
    let t51325 = t14092 * t6569;
    let t51328 = t14022 * t828;
    let t51329 = t51328 * t2123;
    (t51309, t51312, t51315, t51317, t51325, t51329)
}
