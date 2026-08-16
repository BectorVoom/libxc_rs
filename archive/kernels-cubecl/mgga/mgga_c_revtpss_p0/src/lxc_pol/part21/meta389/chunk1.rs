//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1832/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1832<F: Float>(t1248: F, t3584: F, t1287: F, t12233: F, t12240: F, t12242: F, t12245: F, t12251: F, t12360: F, t12363: F, t12573: F, t12575: F, t12577: F, t12598: F) -> (F, F, F) {
    let t12726 = t3584 * t1248;
    let t12727 = t12726 * t1287;
    let t12730 = t12240 + t12242 + t12245 - t12251 + t12360 + t12233 - t12598 - t12575 - t12577 - t12573 - t12363;
    (t12726, t12727, t12730)
}
