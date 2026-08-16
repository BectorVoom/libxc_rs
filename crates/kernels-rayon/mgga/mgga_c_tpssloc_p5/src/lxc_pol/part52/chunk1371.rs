//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1371/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1371(t25992: f64, t8690: f64, t24991: f64, t119837: f64, t119839: f64, t119841: f64, t119844: f64, t119845: f64, t119850: f64, t119852: f64, t119856: f64, t24983: f64, t27290: f64, t4026: f64, t6517: f64, t7266: f64, t8682: f64) -> f64 {
    let t123027 = t8690 * t25992;
    let t123028 = t8690 * t24991;
    let t123034 = -2.0_f64 * t24983 * t7266 - 2.0_f64 * t27290 * t6517 - t4026 * t8682 - t119837 - t119839 - t119841 - t119844 - t119845 - t119850 - t119852 - t119856 - t123027 + 3.0_f64 * t123028;
    t123034
}
