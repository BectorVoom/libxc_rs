//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1819/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1819<F: Float>(t3173: F, t7122: F, t2269: F, t343: F, t136: F, t1007: F, t7106: F, t1968: F, t3080: F, t7105: F, t800: F) -> (F, F, F, F, F, F) {
    let t25529 = t7122 * t3173;
    let t25531 = t2269 * t343;
    let t25532 = t25531 * t136;
    let t25535 = t7106 * t1007;
    let t25538 = t1968 * t3080 / F::cast_from(432.0_f64);
    let t25539 = t7105 * t800;
    (t25529, t25531, t25532, t25535, t25538, t25539)
}
