//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1310/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1310<F: Float>(t1470: F, t21663: F, t1497: F, t5868: F, t77: F, t108772: F, t108782: F, t108995: F, t1928: F, t28127: F, t28138: F, t29526: F, t29529: F, t29533: F, t29538: F, t29544: F, t6958: F, t7706: F, t7709: F, t7716: F, t7720: F) -> F {
    let t114270 = t21663 * t1470;
    let t114288 = t77 * t5868 * t1497;
    let t114292 = F::new(5.0) / F::new(2.0) * t108995 * t7706 + t114270 * t1928 + F::new(5.0) * t108772 * t7706 + F::new(2.0) * t29538 * t7716 + F::new(5.0) * t28138 * t29544 + F::new(2.0) * t29538 * t7720 + F::new(5.0) / F::new(2.0) * t108782 * t7706 + t7709 * t29526 + F::new(5.0) * t28127 * t29544 + F::new(2.0) * t7709 * t29529 + F::new(5.0) / F::new(2.0) * t6958 * t114288 + t7709 * t29533;
    t114292
}
