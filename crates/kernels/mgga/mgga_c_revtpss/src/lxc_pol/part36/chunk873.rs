//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 873/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk873<F: Float>(t1904: F, t3895: F, t2439: F, t1532: F, t2609: F, t2626: F, t4398: F, t2516: F, t2496: F, t2619: F, t4302: F, t123: F, t1534: F) -> (F, F, F, F, F, F, F) {
    let t14296 = t3895 * t1904;
    let t14297 = t2439 * t14296;
    let t14312 = t1532 * t2609;
    let t14328 = t4398 * t2626;
    let t14334 = t4398 * t2516;
    let t14336 = t4398 * t2496;
    let t14339 = t4302 * t2619;
    let t14362 = t1534 * t123;
    (t14297, t14312, t14328, t14334, t14336, t14339, t14362)
}
