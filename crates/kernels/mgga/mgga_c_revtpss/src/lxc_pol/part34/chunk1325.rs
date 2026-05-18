//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1325/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1325<F: Float>(t114536: F, t114556: F, t114570: F, t114588: F, t108249: F, t108251: F, t108280: F, t108294: F, t108296: F, t108302: F, t108308: F, t114477: F, t2022: F, t2027: F, t2028: F, t22953: F, t543: F, t545: F, t7295: F, t7301: F, t94648: F, t97823: F, t97825: F, t97847: F, t97882: F) -> (F, F) {
    let t114590 = t114536 + t114556 + t114570 + t114588;
    let t114611 = -F::new(0.21951497276451705329e-1) * t97823 + F::new(0.39029762157531132076e-1) * t97825 - F::new(0.21684070470512998656e-1) * t108249 + F::new(0.38554277296572111609e-1) * t108251 + t94648 - F::new(0.4336814094102599731e0) * t2027 * t2028 * t545 * t114590 - F::new(0.38554277296572111609e-1) * t108280 + F::new(0.4336814094102599731e0) * t7295 * t7301 * t2022 * t22953 * t543 + F::new(0.13010442282307799193e1) * t7295 * t7301 * t114477 * t543 + F::new(0.14456046980341999104e-2) * t97847 + F::new(0.13010442282307799194e0) * t108294 - F::new(0.23132566377943266966e0) * t108296 - F::new(0.16463622957338778996e-1) * t108302 - F::new(0.38554277296572111609e-1) * t108308 - F::new(0.34697458558045176417e-2) * t97882;
    (t114590, t114611)
}
