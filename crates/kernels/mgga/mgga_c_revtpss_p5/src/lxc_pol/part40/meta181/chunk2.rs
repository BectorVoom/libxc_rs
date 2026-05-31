//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 784/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk784<F: Float>(t1398: F, t543: F, t550: F, t3992: F, t2661: F, t1384: F, t544: F, t235: F) -> (F, F, F, F, F) {
    let t3994 = t550 * t1398 * t543;
    let t3995 = t3992 * t3994;
    let t3996 = t2661 * t3995;
    let t3999 = F::cast_from(1.0_f64) / t1384 / t544;
    let t4000 = t3999 * t235;
    (t3994, t3995, t3996, t3999, t4000)
}
