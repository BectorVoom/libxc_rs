//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1240/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1240<F: Float>(t4480: F, t5728: F, t4484: F, t1705: F, t4487: F, t935: F, t5570: F, t6259: F, t1232: F, t1656: F, t520: F, t1265: F, t1640: F) -> (F, F, F, F, F, F, F) {
    let t19493 = t5728 * t4480;
    let t19495 = t5728 * t4484;
    let t19506 = t1705 * t4487;
    let t19507 = t19506 * t935;
    let t19509 = t6259 * t5570;
    let t19521 = t1656 * t1232 * t520;
    let t19535 = t1640 * t1265;
    (t19493, t19495, t19506, t19507, t19509, t19521, t19535)
}
