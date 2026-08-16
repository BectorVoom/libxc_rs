//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta289 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1000;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1001;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1002;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1003;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta289(t21138: f64, t908: f64, t136: f64, t4362: f64, t5705: f64, t4378: f64, t10564: f64, t21130: f64, t123: f64, t21118: f64, t2768: f64, t882: f64, t21134: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21139, t21140, t21142, t21144, t21146, t21147) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1000(t21138, t908, t136, t4362, t5705, t4378, t10564, t21130, t123);
        let (t21149, t21150) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1001(t21118, t2768, t123);
        let (t21152, t21153) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1002(t21138, t882, t123);
        let (t21155, t21156) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1003(t21134, t882, t123);
    (t21139, t21140, t21142, t21144, t21146, t21147, t21149, t21150, t21152, t21153, t21155, t21156)
}
