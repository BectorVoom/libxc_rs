//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta190 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk982;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk983;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk984;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk985;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk986;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk987;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk988;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk989;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk990;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk991;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta190(t4234: f64, t819: f64, t820: f64, t4180: f64, t4181: f64, t829: f64, t120: f64, t1484: f64, t2645: f64, t1516: f64, t2697: f64, t776: f64, t2701: f64, t4119: f64, t847: f64, t2621: f64, t2623: f64, t2640: f64, t2643: f64, t2695: f64, t2698: f64, t4191: f64, t817: f64, t843: f64, t4189: f64, t218: f64, t1520: f64, t225: f64, t1527: f64, t865: f64, t2718: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4236 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk982(t4234, t819, t820);
        let t4240 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk983(t4180, t4181, t829);
        let (t4248, t4250) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk984(t120, t1484, t2645, t829);
        let (t4253, t4255) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk985(t1516, t2697, t1484, t776);
        let t4257 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk986(t2701, t4255, t820);
        let t4261 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk987(t4119, t820, t847);
        let t4264 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk988(t1516, t2621, t2623, t2640, t2643, t2695, t2698, t4191, t4236, t4240, t4250, t4253, t4257, t4261, t817, t843);
        let t4265 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk989(t4189, t4264);
        let (t4266, t4268) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk990(t218, t4265, t1520, t225);
        let (t4272, t4273) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk991(t1527, t865, t2718);
    (t4236, t4240, t4248, t4250, t4255, t4257, t4261, t4265, t4266, t4268, t4272, t4273)
}
