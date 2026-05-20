//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1448/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1448<F: Float>(t18353: F, t2689: F, t18348: F, t2710: F, t2713: F, t18562: F, t2626: F, t2609: F, t5944: F, t10815: F, t5980: F, t40398: F, t6024: F) -> (F, F, F, F, F, F) {
    let t62129 = t2689 * t18353;
    let t62251 = t2710 * t2713 * t18348;
    let t62276 = t18562 * t2626;
    let t62300 = t5944 * t2609;
    let t62399 = t10815 * t5980;
    let t62401 = t40398 * t6024;
    (t62129, t62251, t62276, t62300, t62399, t62401)
}
