//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1004/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1004(t3180: f64, t6711: f64, t3134: f64, t6538: f64, t6188: f64, t343: f64, t8840: f64, t337: f64, t2121: f64, t2134: f64, t6445: f64, t6447: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9021 = t6711 * t3180 / 48.0_f64;
    let t9023 = t6538 * t3134 / 96.0_f64;
    let t9025 = t6188 * t3134 / 96.0_f64;
    let t9026 = t8840 * t343;
    let t9027 = t337 * t9026;
    let t9028 = t2121 * t9027;
    let t9030 = t2134 * t9028 / 48.0_f64;
    let t9031 = 7.0_f64 / 288.0_f64 * t6445;
    let t9032 = 7.0_f64 / 288.0_f64 * t6447;
    (t9021, t9023, t9025, t9026, t9030, t9031, t9032)
}
