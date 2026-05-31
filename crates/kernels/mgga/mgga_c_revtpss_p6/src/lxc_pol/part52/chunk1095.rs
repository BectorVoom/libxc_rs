//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1095/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1095<F: Float>(t34216: F, t34240: F, t532: F, t1450: F, t2014: F, t1519: F, t1843: F, t1932: F, t2089: F, t2108: F, t32389: F, t33913: F, t34168: F, t34188: F, t34191: F, t34193: F, t34195: F, t34198: F, t34203: F, t508: F, t7725: F, t8065: F, t8109: F, t8568: F, t8627: F) -> (F, F, F, F) {
    let t34241 = t34216 + t34240;
    let t34242 = t532 * t34241;
    let t34243 = t34242 * t1450;
    let t34244 = t2014 * t34243;
    let t34245 = -F::cast_from(2.0_f64) * t1519 * t32389 - t1843 * t8627 - t1932 * t8065 - t2089 * t7725 + t2108 * t33913 - t34188 * t508 + t8109 * t8568 - t34168 + t34191 - t34193 - t34195 - t34198 + t34203 + t34244;
    (t34241, t34242, t34243, t34245)
}
