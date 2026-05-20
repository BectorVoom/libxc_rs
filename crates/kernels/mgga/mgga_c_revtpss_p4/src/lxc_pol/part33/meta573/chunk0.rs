//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1982/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1982<F: Float>(t2246: F, t5812: F, t10871: F, t5977: F, t18493: F, t221: F, t18498: F, t6016: F, t836: F, t18435: F, t6022: F, t23160: F) -> (F, F, F, F, F, F, F, F, F) {
    let t60673 = t5812 * t2246;
    let t61532 = t5977 * t10871;
    let t61639 = t221 * t18493;
    let t61725 = t221 * t18498;
    let t61749 = t6016 * t836;
    let t61756 = t5977 * t836;
    let t62403 = t221 * t18435;
    let t62589 = t61532 * t836;
    let t62593 = t6022 * t836;
    let t62604 = t23160 * t836;
    (t60673, t61639, t61725, t61749, t61756, t62403, t62589, t62593, t62604)
}
