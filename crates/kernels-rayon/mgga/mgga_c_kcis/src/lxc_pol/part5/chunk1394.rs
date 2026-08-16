//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1394/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1394(t187: f64, t6996: f64, t1357: f64, t1585: f64, t21172: f64, t21174: f64, t21176: f64, t21178: f64, t21180: f64, t21293: f64, t21295: f64, t21320: f64, t21324: f64, t21330: f64, t21334: f64, t21449: f64, t22842: f64, t22884: f64, t22927: f64, t22983: f64, t4381: f64, t5615: f64, t6125: f64, t7004: f64, t7025: f64) -> f64 {
    let t22989 = t187 * t6996;
    let t23006 = t187 * (t22842 + t22884 + t22927 + t22983) + t21172 + t21174 + t21176 - t21178 + t21180 + t21293 + t21295 + 0.11696446794910408142e1_f64 * t4381 * t7004 - 0.58482233974552040708e0_f64 * t22989 * t1357 - 0.58482233974552040708e0_f64 * t1585 * t21449 - 0.17315755899375863299e2_f64 * t4381 * t7025 - 0.11696446794910408142e1_f64 * t6125 * t5615 - 0.17315755899375863299e2_f64 * t1585 * t21334 - 0.34631511798751726598e2_f64 * t1585 * t21330 + 0.1038945353962551798e3_f64 * t1585 * t21320 + 0.11696446794910408142e1_f64 * t1585 * t21324;
    t23006
}
