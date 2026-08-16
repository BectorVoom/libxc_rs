//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1177/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1177(t1208: f64, t5295: f64, t2035: f64, t5266: f64, t1196: f64, t1701: f64, t80012: f64, t1111: f64, t1197: f64, t1201: f64, t1209: f64, t1472: f64, t14738: f64, t19039: f64, t19135: f64, t22063: f64, t22090: f64, t2691: f64, t292: f64, t4104: f64, t5265: f64, t88456: f64, t88911: f64) -> (f64, f64, f64, f64) {
    let t90062 = t1208 * t5295;
    let t90075 = t2035 * t5266 * t5295;
    let t90081 = t1701 * t80012 * t1196;
    let t90085 = t1701 * t80012 * t1208;
    let t90088 = -0.89366407315441549491e3_f64 * t1201 * t88911 + 0.44683203657720774746e3_f64 * t292 * t88911 + 0.23380572188451859703e3_f64 * t1201 * t88456 + 48.0_f64 * t2691 * t14738 * t90062 - 0.23380572188451859703e3_f64 * t292 * t88456 - 0.11093760908123778558e3_f64 * t19039 * t22090 * t1197 + 0.55468804540618892788e2_f64 * t5265 * t22090 * t1209 - 0.87582322958871935983e1_f64 * t19135 * t90075 + 0.14498192132169191472e2_f64 * t22063 * t1111 + 0.22445349300913785316e3_f64 * t4104 * t90081 - 0.11222674650456892658e3_f64 * t1472 * t90085;
    (t90075, t90081, t90085, t90088)
}
