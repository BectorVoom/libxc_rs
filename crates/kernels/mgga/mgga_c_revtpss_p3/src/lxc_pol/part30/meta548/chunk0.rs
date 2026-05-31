//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1987/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1987<F: Float>(t10115: F, t555: F, t4146: F, t1398: F, t21990: F, t13790: F, t4056: F, t1882: F, t3923: F, t4003: F, t9994: F, t13872: F, t221: F) -> (F, F, F, F, F, F, F, F) {
    let t47567 = t10115 * t555;
    let t47671 = t4146 * t4146;
    let t47672 = F::cast_from(1.0_f64) / t47671;
    let t48020 = t21990 * t1398;
    let t48025 = t13790 * t4056;
    let t48073 = t1882 * t3923;
    let t48074 = t48073 * t4003;
    let t48105 = t1882 * t9994;
    let t48141 = t221 * t13872;
    (t47567, t47672, t48020, t48025, t48073, t48074, t48105, t48141)
}
