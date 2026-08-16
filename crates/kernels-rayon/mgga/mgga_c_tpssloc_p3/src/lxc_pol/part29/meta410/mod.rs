//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1664;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1665;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1666;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta410(t1307: f64, t1388: f64, t118: f64, t1787: f64, t2375: f64, t12045: f64, t12050: f64, t12052: f64, t12054: f64, t5151: f64, t750: f64, t17: f64, t12089: f64, t12091: f64, t12044: f64, t12048: f64, t12057: f64, t12059: f64, t12087: f64, t12094: f64, t3734: f64, t3918: f64, t3919: f64, t5122: f64, t5126: f64, t5161: f64, t5187: f64, t5308: f64, t9789: f64, t9793: f64, t25: f64, t12061: f64, t1408: f64, t2: f64, t3664: f64, t584: f64, t606: f64, t16: f64, t2249: f64, t3665: f64, t5134: f64, t5137: f64, t514: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15904, t15910, t15911, t15915, t15916, t15917, t15923) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1664(t1307, t1388, t118, t1787, t2375, t12045, t12050, t12052, t12054, t5151, t750, t17);
        let (t15927, t15928, t15929) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1665(t12089, t12091, t12044, t12048, t12057, t12059, t12087, t12094, t15904, t15910, t15911, t15915, t15916, t15917, t15923, t3734, t3918, t3919, t5122, t5126, t5161, t5187, t5308, t9789, t9793);
        let (t15941, t15951) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1666(t25, t12061, t1408, t2, t3664, t584, t606, t16, t2249, t3665, t5134, t5137, t514, zeta_threshold);
    (t15904, t15910, t15911, t15915, t15916, t15917, t15923, t15927, t15928, t15929, t15941, t15951)
}
