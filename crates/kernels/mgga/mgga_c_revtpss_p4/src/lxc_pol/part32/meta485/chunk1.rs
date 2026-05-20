//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1731/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1731<F: Float>(t25826: F, t28036: F, t4287: F, t6998: F, t4237: F, t76: F, t13269: F, t38: F, t1497: F, t640: F, t77: F, t4241: F, t84: F) -> (F, F, F, F, F, F) {
    let t28037 = t25826 * t28036;
    let t28039 = t6998 * t4287;
    let t28089 = t76 * t4237;
    let t28093 = t13269 * t38;
    let t28104 = t640 * t1497;
    let t28105 = t77 * t28104;
    let t28108 = t84 * t4241;
    (t28037, t28039, t28089, t28093, t28105, t28108)
}
