//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2031/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2031<F: Float>(t2482: F, t25981: F, t27: F, t10003: F, t25997: F, t9970: F, t550: F, t7021: F, t3946: F, t25273: F, t540: F, t1372: F) -> (F, F, F, F, F) {
    let t94508 = t2482 * t25981 * t27;
    let t94509 = t94508 * t10003;
    let t94511 = t25997 * t9970;
    let t94513 = t7021 * t550;
    let t94514 = t94513 * t3946;
    let t94519 = t25273 * t540;
    let t94520 = t94519 * t1372;
    (t94509, t94511, t94514, t94519, t94520)
}
