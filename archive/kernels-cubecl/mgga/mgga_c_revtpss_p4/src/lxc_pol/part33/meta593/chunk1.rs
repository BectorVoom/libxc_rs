//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2010/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2010<F: Float>(t4021: F, t94497: F, t2482: F, t25981: F, t27: F, t550: F, t7021: F, t25273: F, t540: F, t1372: F, t2019: F, t9951: F) -> (F, F, F, F, F, F) {
    let t94498 = t94497 * t4021;
    let t94508 = t2482 * t25981 * t27;
    let t94513 = t7021 * t550;
    let t94519 = t25273 * t540;
    let t94520 = t94519 * t1372;
    let t94522 = t2019 * t9951;
    (t94498, t94508, t94513, t94519, t94520, t94522)
}
