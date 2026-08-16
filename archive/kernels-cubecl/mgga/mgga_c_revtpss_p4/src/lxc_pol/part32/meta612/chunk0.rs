//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1951/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1951<F: Float>(t1711: F, t4537: F, t25759: F, t77408: F, t6416: F, t890: F, t1113: F, t5966: F, t6075: F, t106610: F, t27799: F, t18435: F, t27763: F) -> (F, F, F, F, F, F, F) {
    let t107988 = t1711 * t4537;
    let t108002 = t25759 * t77408;
    let t108005 = t6416 * t890;
    let t108009 = t1113 * t5966;
    let t108021 = t1113 * t6075;
    let t108028 = t27799 * t106610;
    let t108030 = t27763 * t18435;
    (t107988, t108002, t108005, t108009, t108021, t108028, t108030)
}
