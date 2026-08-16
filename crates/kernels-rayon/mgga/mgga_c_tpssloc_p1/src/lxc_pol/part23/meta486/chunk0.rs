//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1490/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1490(t25: f64, t54312: f64, t54314: f64, t54316: f64, t6305: f64, t5397: f64, t19547: f64, t20216: f64, t3664: f64, t39419: f64, t5134: f64, t514: f64, t75911: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t79856 = 96.0_f64 * t54312;
    let t79857 = 576.0_f64 * t54314;
    let t79858 = 384.0_f64 * t54316;
    let t79859 = t6305 * t6305;
    let t79864 = t5397 * t5397;
    let t79872 = piecewise3(t26, 0.0_f64, 40.0_f64 / 81.0_f64 * t39419 * t79859 - 16.0_f64 / 9.0_f64 * t19547 * t5397 + 4.0_f64 / 3.0_f64 * t3664 * t79864 + 16.0_f64 / 9.0_f64 * t5134 * t20216 + 4.0_f64 / 3.0_f64 * t514 * t75911);
    (t79856, t79857, t79858, t79859, t79864, t79872)
}
