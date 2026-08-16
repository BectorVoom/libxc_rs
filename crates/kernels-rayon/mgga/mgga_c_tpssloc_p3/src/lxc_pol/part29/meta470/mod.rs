//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1802;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1803;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1804;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1805;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta470(t2752: f64, t6665: f64, t10143: f64, t1914: f64, t25: f64, t2749: f64, t606: f64, t868: f64, t2745: f64, t1877: f64, t1915: f64, t2249: f64, t22951: f64, t22959: f64, t22961: f64, t22964: f64, t22968: f64, t23286: f64, t2522: f64, t4314: f64, t6542: f64, t6666: f64, t6670: f64, t6671: f64, t134: f64, t221: f64, t2250: f64, t3: f64, t3034: f64, t371: f64, t13487: f64, t193: f64, t202: f64, t23285: f64, t2379: f64, t2553: f64, t776: f64, t870: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t23290 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1802(t2752, t6665);
        let t23295 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1803(t10143, t1914);
        let (t23296, t23299, t23302, t23309) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1804(t25, t2749, t606, t868, t2745, t1877, t1915, t2249, t22951, t22959, t22961, t22964, t22968, t23286, t23290, t23295, t2522, t4314, t6542, t6666, t6670, t6671);
        let (t23383, t23413, t23508, t23598, t23772) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1805(t134, t221, t2250, t3, t3034, t371, t13487, t1877, t1915, t193, t202, t23285, t23290, t23295, t2379, t2522, t2553, t2745, t2749, t4314, t6666, t6670, t776, t868, t870);
    (t23290, t23295, t23296, t23299, t23302, t23309, t23383, t23413, t23508, t23598, t23772)
}
