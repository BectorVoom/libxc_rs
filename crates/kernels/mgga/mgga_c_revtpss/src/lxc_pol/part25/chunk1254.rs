//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1254/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1254<F: Float>(t10638: F, t1949: F, t1959: F, t231: F, t25317: F, t25319: F, t25349: F, t25383: F, t25392: F, t2645: F, t27353: F, t2771: F, t39620: F, t7048: F, t7070: F, t7076: F, t93206: F, t93207: F, t93210: F, t93224: F, t93226: F, t93228: F, t93231: F, t93242: F, t93244: F) -> F {
    let t93250 = t93206 - F::new(0.39029762157531132076e-1) * t93207 - t93210 + F::new(0.13010442282307799193e1) * t7070 * t7076 * t7048 * t2645 * t231 + F::new(0.4336814094102599731e0) * t7070 * t7076 * t1949 * t10638 * t231 + F::new(0.13010442282307799193e1) * t25383 * t25349 + t93224 - F::new(0.21684070470512998656e-1) * t93226 + F::new(0.38554277296572111609e-1) * t93228 - t93231 - F::new(0.78062653693846795158e1) * t25383 * t25319 - F::new(0.78062653693846795158e1) * t7070 * t25317 * t7048 * t2771 + F::new(0.72280234901709995519e-3) * t93242 - F::new(0.4336814094102599731e0) * t93244 * t1959 + F::new(0.13010442282307799193e1) * t27353 * t25392 * t39620;
    t93250
}
