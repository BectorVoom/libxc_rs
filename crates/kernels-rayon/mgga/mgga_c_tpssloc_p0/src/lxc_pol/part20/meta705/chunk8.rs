//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2686/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2686(t1307: f64, t16094: f64, t54665: f64, t686: f64, t16095: f64, t3719: f64, t2559: f64, t5194: f64, t5198: f64, t118: f64, t16018: f64, t3739: f64, t794: f64) -> (f64, f64, f64, f64) {
    let t54690 = t16094 * t686 * t54665 * t1307;
    let t54698 = t16094 * t686 * t16095 * t3719;
    let t54701 = t2559 * t5194 * t5198;
    let t54702 = 0.11666666666666666666e0_f64 * t54701;
    let t54705 = t3739 * t118 * t794 * t16018;
    (t54690, t54698, t54702, t54705)
}
