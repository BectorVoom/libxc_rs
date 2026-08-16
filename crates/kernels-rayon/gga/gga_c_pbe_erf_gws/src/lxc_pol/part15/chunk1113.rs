//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1113/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1113(t14064: f64, t854: f64, t2308: f64, t4039: f64, t3065: f64, t876: f64, t2134: f64, t1189: f64, t2334: f64, t2285: f64, t4043: f64, t2293: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14065 = t854 * t14064;
    let t14067 = t4039 * t2308;
    let t14069 = t3065 * t876;
    let t14070 = t2134 * t14069;
    let t14072 = t1189 * t2334;
    let t14073 = 119.0_f64 / 6912.0_f64 * t14072;
    let t14074 = t4043 * t2285;
    let t14076 = t4043 * t2293;
    (t14065, t14067, t14069, t14070, t14073, t14074, t14076)
}
