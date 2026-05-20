//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2254/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2254<F: Float>(t2247: F, t4187: F, t10309: F, t1470: F, t101227: F, t101230: F, t101234: F, t101237: F, t101240: F, t25147: F, t25150: F, t25162: F, t25164: F, t28154: F, t7702: F, t7716: F, t92570: F, t92573: F, t92577: F, t92585: F, t92690: F) -> F {
    let t101243 = t2247 * t4187;
    let t101252 = t10309 * t1470;
    let t101259 = -F::new(10.0) / F::new(3.0) * t25162 * t101227 - F::new(10.0) / F::new(3.0) * t101230 * t25164 + F::new(35.0) * t92690 * t101234 - F::new(10.0) / F::new(3.0) * t101237 * t25164 - F::new(10.0) / F::new(3.0) * t101240 * t25164 - F::new(10.0) / F::new(3.0) * t101243 * t25164 - F::new(10.0) / F::new(3.0) * t28154 * t92573 - F::new(10.0) / F::new(3.0) * t28154 * t92577 - F::new(5.0) / F::new(3.0) * t28154 * t92585 + F::new(10.0) * t101252 * t92570 - t7702 * t25147 / F::new(6.0) - t25150 * t7716 / F::new(6.0);
    t101259
}
