//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 456/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk456<F: Float>(t1592: F, t904: F, t128: F, t903: F, t291: F, t902: F) -> (F, F, F, F, F) {
    let t1593 = t904 * t1592;
    let t1594 = t128 * t1593;
    let t1596 = -t903 - F::cast_from(0.17808333333333333333e-1_f64) * t1594;
    let t1598 = F::cast_from(0.621814e-1_f64) * t1596 * t291;
    let t1600 = -t902 / F::cast_from(3.0_f64) - t1594 / F::cast_from(3.0_f64);
    (t1593, t1594, t1596, t1598, t1600)
}
