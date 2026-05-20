//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1153/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1153<F: Float>(t241: F, t25981: F, t820: F, t25997: F, t5665: F, t1873: F, t26004: F, t1904: F, t7242: F, t689: F, t786: F, t7911: F) -> (F, F, F, F, F, F) {
    let t27940 = t820 * t25981 * t241;
    let t27953 = t25997 * t5665;
    let t27955 = t26004 * t1873;
    let t27965 = t7242 * t1904;
    let t27966 = t689 * t27965;
    let t27968 = t786 * t7911;
    (t27940, t27953, t27955, t27965, t27966, t27968)
}
