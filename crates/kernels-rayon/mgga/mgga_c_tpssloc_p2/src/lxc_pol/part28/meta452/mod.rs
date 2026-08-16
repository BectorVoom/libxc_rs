//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta452 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1647;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1648;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta452(t24115: f64, t24137: f64, t1378: f64, t1323: f64, t7191: f64, t1385: f64, t7213: f64, t3887: f64, t22923: f64, t22925: f64, t2085: f64, t3752: f64, t1375: f64, t22664: f64, t22668: f64, t22676: f64, t22688: f64, t22907: f64, t22909: f64, t22918: f64, t22921: f64, t22928: f64, t22931: f64, t22936: f64, t22940: f64, t568: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24138, t24139, t24141, t24147, t24156, t24157, t24162) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1647(t24115, t24137, t1378, t1323, t7191, t1385, t7213, t3887, t22923, t22925, t2085, t3752);
        let t24164 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1648(t1375, t22664, t22668, t22676, t22688, t22907, t22909, t22918, t22921, t22928, t22931, t22936, t22940, t24139, t24141, t24147, t24156, t24157, t24162, t568);
    (t24138, t24139, t24141, t24147, t24156, t24157, t24162, t24164)
}
