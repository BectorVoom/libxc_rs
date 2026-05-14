//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1068/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1068<F: Float>(t30194: F, t572: F, t1918: F, t2040: F, t30171: F, t30180: F, t30182: F, t30184: F, t30187: F, t30190: F, t30193: F, t573: F, t6945: F, t6948: F, t7944: F, t1518: F, t1931: F) -> (F, F) {
    let t30196 = 3.0 * t572 * t30194;
    let t30197 = 6.0 * t1918 * t7944 + 6.0 * t2040 * t6945 + 3.0 * t2040 * t6948 + t30171 * t573 + t30180 + t30182 + t30184 + t30187 + t30190 + t30193 + t30196;
    let t33602 = t1931 * t1518;
    (t30197, t33602)
}
