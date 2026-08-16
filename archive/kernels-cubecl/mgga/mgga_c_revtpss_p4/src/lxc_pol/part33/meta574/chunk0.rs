//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1983/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1983<F: Float>(t1559: F, t4423: F, t14586: F, t231: F, t61749: F, t61756: F, t1544: F, t2411: F, t22461: F, t4147: F, t6861: F, t9994: F) -> (F, F, F, F, F, F, F) {
    let t62624 = t1559 * t4423;
    let t62628 = t14586 * t4423;
    let t62637 = t61749 * t231;
    let t62695 = t61756 * t231;
    let t63185 = t2411 * t1544;
    let t73407 = t22461 * t4147;
    let t73820 = t6861 * t9994;
    (t62624, t62628, t62637, t62695, t63185, t73407, t73820)
}
