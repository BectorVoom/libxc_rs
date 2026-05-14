//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 967/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk967<F: Float>(t12426: F, t12559: F, t300: F, t12224: F, t12233: F, t12237: F, t12240: F, t12242: F, t12245: F, t12251: F, t12360: F, t12363: F, t12366: F, t12381: F, t12395: F, t1179: F, t1188: F, t12547: F) -> (F, F, F) {
    let t12561 = t300 * (t12426 + t12559);
    let t12562 = -t12224 + t12233 + t12237 + t12240 + t12242 + t12245 - t12251 + t12360 - t12363 + t12366 + t12381 - t12395 + t12561;
    let t12564 = t1179 * t12547 * t1188;
    (t12561, t12562, t12564)
}
