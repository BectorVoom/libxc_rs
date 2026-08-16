//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1509/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1509(t54460: f64, t54462: f64, t54467: f64, t57235: f64, t54477: f64, t39655: f64, t39658: f64, t39660: f64, t39844: f64, t39856: f64, t40224: f64, t40228: f64, t40230: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t80112 = 960.0_f64 * t54460;
    let t80113 = 480.0_f64 * t54462;
    let t80114 = 0.4101607543286562663e4_f64 * t54467;
    let t80115 = 0.65061487801810439052e-1_f64 * t57235;
    let t80116 = 48.0_f64 * t54477;
    let t80117 = -t39655 - t39658 + t39660 + t39844 - t80112 - t80113 - t39856 - t80114 + t40224 + t40228 - t40230 + t80115 - t80116;
    (t80112, t80113, t80114, t80115, t80116, t80117)
}
