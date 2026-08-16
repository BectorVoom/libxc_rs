//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 988/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk988(t1066: f64, t3515: f64, t218: f64, t219: f64, t10767: f64, t208: f64, t10769: f64, t10801: f64, t10803: f64, t10807: f64, t10812: f64, t10814: f64, t10816: f64, t5543: f64, t5558: f64, t7332: f64, t7357: f64, t9148: f64, t9185: f64, t9192: f64) -> (f64, f64, f64, f64, f64) {
    let t10821 = t1066 * t3515;
    let t10823 = t218 * t219 * t10821;
    let t10825 = t208 * t10767;
    let t10827 = t218 * t219 * t10825;
    let t10829 = 0.19419375e1_f64 * t10801 - 0.3883875e1_f64 * t10803 + 0.258925e1_f64 * t10807 - t5543 + 0.12077e1_f64 * t7357 - 0.905775e0_f64 * t9148 + 0.905775e0_f64 * t10769 - 0.412621875e-1_f64 * t10812 + 0.247573125e0_f64 * t10814 + 0.16504875e0_f64 * t10816 - t5558 + 0.82785e0_f64 * t7332 - 0.49671e0_f64 * t9185 - 0.49671e0_f64 * t9192 + 0.745065e0_f64 * t10823 + 0.248355e0_f64 * t10827;
    (t10821, t10823, t10825, t10827, t10829)
}
