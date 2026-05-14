//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 636/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk636<F: Float>(t222: F, t227: F, t1056: F, t5562: F, t5565: F, t967: F, t2063: F, t3289: F, t220: F, t229: F, t1060: F, t44: F, t3281: F, t295: F, t119: F, t79: F, t1337: F, t140: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t223 = t222 <= zeta_threshold;
    let t228 = t227 <= zeta_threshold;
    let t5569 = piecewise3(t223, 0.0, 4.0 / 9.0 * t5562 * t1056 + 8.0 / 3.0 * t5565 * t967);
    let t5570 = t3289 * t2063;
    let t5573 = t229 * t220;
    let t5577 = piecewise3(t228, 0.0, 4.0 / 9.0 * t5570 * t1060 - 8.0 / 3.0 * t5573 * t967);
    let t5579 = (t5569 + t5577) * t44;
    let t5584 = 2.0 * t3281;
    let t5585 = piecewise3(t223, 0.0, t5584);
    let t5586 = t295 * t5585;
    let t5598 = t119 * t79;
    let t5600 = t140 * t5598 * t1337;
    (t5570, t5573, t5579, t5585, t5586, t5598, t5600)
}
