//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta152 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk804;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk805;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk806;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta152(t1489: f64, t2563: f64, t131: f64, t2570: f64, t205: f64, t1484: f64, t213: f64, t221: f64, t776: f64, t118: f64, t794: f64, t2576: f64, t210: f64, t214: f64, t4119: f64, t2562: f64, t2564: f64, t2569: f64, t2579: f64, t2590: f64, t787: f64, t252: f64, t1492: f64, t852: f64, t1493: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4124, t4126, t4127, t4128, t4130, t4134, t4135) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk804(t1489, t2563, t131, t2570, t205, t1484, t213, t221, t776, t118, t794, t2576);
        let (t4138, t4142) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk805(t210, t214, t4119, t2562, t2564, t2569, t2579, t2590, t4124, t4127, t4130, t4135, t787);
        let (t4143, t4145, t4147) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk806(t252, t4142, t1492, t852, t1493, t225);
    (t4124, t4126, t4127, t4128, t4130, t4134, t4135, t4138, t4142, t4143, t4145, t4147)
}
