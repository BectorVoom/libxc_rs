//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 943/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk943<F: Float>(t12224: F, t12237: F, t12366: F, t12381: F, t12395: F, t12413: F, t12417: F, t12561: F, t12566: F, t12579: F, t12583: F, t12594: F, t12730: F, t1287: F, t487: F, t12646: F, t1280: F) -> (F, F, F) {
    let t12731 = t12237 + t12366 - t12413 + t12417 - t12395 - t12594 - t12224 + t12381 + t12561 + t12579 + t12583 - t12566;
    let t12732 = t12730 + t12731;
    let t12734 = t487 * t12732 * t1287;
    let t12737 = t1280 * t12646;
    (t12732, t12734, t12737)
}
