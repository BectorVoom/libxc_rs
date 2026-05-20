//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1946/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1946<F: Float>(t28640: F, t6963: F, t28141: F, t7349: F, t101350: F, t10309: F, t25120: F, t26172: F, t28147: F, t33269: F, t7343: F, t7709: F, t7964: F, t95230: F, t95241: F, t95243: F, t95246: F, t95248: F, t95253: F) -> F {
    let t101811 = F::new(32.0) / F::new(9.0) * t6963 * t28640;
    let t101820 = F::new(32.0) / F::new(9.0) * t28141 * t7349;
    let t101824 = -F::new(2.0) / F::new(3.0) * t7709 * t26172 - F::new(5.0) / F::new(3.0) * t7343 * t101350 + t101811 - F::new(2.0) / F::new(3.0) * t25120 * t7964 - F::new(8.0) / F::new(9.0) * t95230 - F::new(8.0) / F::new(9.0) * t95241 - F::new(16.0) / F::new(9.0) * t95243 + F::new(176.0) / F::new(27.0) * t95246 + F::new(16.0) / F::new(9.0) * t95248 + t101820 - F::new(40.0) * t10309 * t33269 * t28147 - t95253;
    t101824
}
