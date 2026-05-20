//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1194/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1194<F: Float>(t10073: F, t4089: F, t1432: F, t2470: F, t4107: F, t1433: F, t9288: F, t136: F, t1419: F, t2457: F, t3964: F, t225: F, t9646: F) -> (F, F, F, F, F) {
    let t10074 = t10073 * t4089;
    let t10098 = t1432 * t4107 * t2470;
    let t10102 = F::cast_from(0.30356481678079769392e-1_f64) * t1432 * t1433 * t9288;
    let t10107 = t1419 * t136;
    let t10109 = t3964 * t10107 * t2457;
    let t10111 = t9646 * t225;
    (t10074, t10098, t10102, t10109, t10111)
}
