//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta110 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk678;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk679;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk680;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk681;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta110(t676: f64, t739: f64, t172: f64, t2368: f64, t2369: f64, t746: f64, t2388: f64, t2391: f64, t2394: f64, t2398: f64, t2400: f64, t2403: f64, t738: f64, t180: f64, t118: f64, t168: f64, t181: f64, t2393: f64, t2408: f64, t2417: f64, t2423: f64, t2426: f64, t2454: f64, t2460: f64, t2462: f64, t2472: f64, t2477: f64, t2480: f64, t2486: f64, t268: f64, t725: f64, t732: f64, t740: f64, t747: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2490, t2494, t2495, t2504) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk678(t676, t739, t172, t2368, t2369, t746, t2388, t2391, t2394, t2398, t2400, t2403);
        let (t2505, t2508, t2509) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk679(t2504, t746, t738);
        let (t2510, t2511, t2512) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk680(t172, t2509, t180);
        let (t2513, t2516) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk681(t2369, t2512, t118, t168, t181, t2393, t2408, t2417, t2423, t2426, t2454, t2460, t2462, t2472, t2477, t2480, t2486, t2490, t2494, t2495, t2505, t2510, t268, t725, t732, t740, t747);
    (t2490, t2494, t2495, t2504, t2505, t2508, t2509, t2510, t2511, t2512, t2513, t2516)
}
