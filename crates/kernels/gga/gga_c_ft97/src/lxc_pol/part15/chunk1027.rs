//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1027/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1027<F: Float>(t1208: F, t5295: F, t2035: F, t5266: F, t1196: F, t1701: F, t80012: F, t1111: F, t1197: F, t1201: F, t1209: F, t1472: F, t14738: F, t19039: F, t19135: F, t22063: F, t22090: F, t2691: F, t292: F, t4104: F, t5265: F, t88456: F, t88911: F) -> (F, F, F, F) {
    let t90062 = t1208 * t5295;
    let t90075 = t2035 * t5266 * t5295;
    let t90081 = t1701 * t80012 * t1196;
    let t90085 = t1701 * t80012 * t1208;
    let t90088 = -0.89366407315441549491e3 * t1201 * t88911 + 0.44683203657720774746e3 * t292 * t88911 + 0.23380572188451859703e3 * t1201 * t88456 + 48.0 * t2691 * t14738 * t90062 - 0.23380572188451859703e3 * t292 * t88456 - 0.11093760908123778558e3 * t19039 * t22090 * t1197 + 0.55468804540618892788e2 * t5265 * t22090 * t1209 - 0.87582322958871935983e1 * t19135 * t90075 + 0.14498192132169191472e2 * t22063 * t1111 + 0.22445349300913785316e3 * t4104 * t90081 - 0.11222674650456892658e3 * t1472 * t90085;
    (t90075, t90081, t90085, t90088)
}
