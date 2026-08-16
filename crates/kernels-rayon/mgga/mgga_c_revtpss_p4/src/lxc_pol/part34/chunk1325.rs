//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1325/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1325(t114536: f64, t114556: f64, t114570: f64, t114588: f64, t108249: f64, t108251: f64, t108280: f64, t108294: f64, t108296: f64, t108302: f64, t108308: f64, t114477: f64, t2022: f64, t2027: f64, t2028: f64, t22953: f64, t543: f64, t545: f64, t7295: f64, t7301: f64, t94648: f64, t97823: f64, t97825: f64, t97847: f64, t97882: f64) -> (f64, f64) {
    let t114590 = t114536 + t114556 + t114570 + t114588;
    let t114611 = -0.21951497276451705329e-1_f64 * t97823 + 0.39029762157531132076e-1_f64 * t97825 - 0.21684070470512998656e-1_f64 * t108249 + 0.38554277296572111609e-1_f64 * t108251 + t94648 - 0.4336814094102599731e0_f64 * t2027 * t2028 * t545 * t114590 - 0.38554277296572111609e-1_f64 * t108280 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t2022 * t22953 * t543 + 0.13010442282307799193e1_f64 * t7295 * t7301 * t114477 * t543 + 0.14456046980341999104e-2_f64 * t97847 + 0.13010442282307799194e0_f64 * t108294 - 0.23132566377943266966e0_f64 * t108296 - 0.16463622957338778996e-1_f64 * t108302 - 0.38554277296572111609e-1_f64 * t108308 - 0.34697458558045176417e-2_f64 * t97882;
    (t114590, t114611)
}
