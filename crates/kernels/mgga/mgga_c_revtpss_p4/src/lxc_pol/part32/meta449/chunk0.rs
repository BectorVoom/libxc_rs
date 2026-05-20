//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1624/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1624<F: Float>(t6864: F, t9918: F, t1353: F, t6816: F, t4012: F, t828: F, t3930: F, t6876: F, t1883: F, t5627: F, t13783: F, t13926: F, t6869: F) -> (F, F, F, F, F, F) {
    let t22285 = t9918 * t6864;
    let t22287 = t6816 * t1353;
    let t22289 = t4012 * t828 * t22287;
    let t22292 = t3930 * t6876;
    let t22294 = t1883 * t5627;
    let t22295 = t13783 * t22294;
    let t22298 = t13926 * t6869;
    (t22285, t22287, t22289, t22292, t22295, t22298)
}
