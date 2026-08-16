//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta121 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk677;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk678;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk679;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta121(t1023: f64, t248: f64, t3101: f64, t1020: f64, t1017: f64, t1030: f64, t1015: f64, t1012: f64, t1009: f64, t990: f64, t1011: f64, t1019: f64, t1004: f64, t1040: f64, t1013: f64, t361: f64, t363: f64, t3037: f64, t3033: f64, t360: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3103, t3104, t3108, t3109, t3112, t3114) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk677(t1023, t248, t3101, t1020, t1017, t1030, t1015, t1012, t1009, t990, t1011, t1019);
        let (t3117, t3127, t3128, t3129, t3130) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk678(t1004, t1040, t1013, t361, t363, t3037, t3033);
        let t3131 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk679(t360);
    (t3103, t3104, t3108, t3109, t3112, t3114, t3117, t3127, t3128, t3129, t3130, t3131)
}
