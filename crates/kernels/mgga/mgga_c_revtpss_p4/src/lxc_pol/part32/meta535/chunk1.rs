//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1844/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1844<F: Float>(t25304: F, t25949: F, t1419: F, t7063: F, t25898: F, t1955: F, t7282: F, t9656: F, t281: F, t555: F, t93238: F, t25877: F) -> (F, F, F, F, F) {
    let t94776 = t25304 * t25949;
    let t94801 = t7063 * t1419;
    let t94802 = t94801 * t25898;
    let t94823 = t1955 * t7282 * t9656;
    let t94849 = t281 * t93238 * t555;
    let t94886 = t94801 * t25877;
    (t94776, t94802, t94823, t94849, t94886)
}
