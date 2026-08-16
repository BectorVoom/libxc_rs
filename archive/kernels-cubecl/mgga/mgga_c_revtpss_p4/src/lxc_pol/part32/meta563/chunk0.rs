//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1884/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1884<F: Float>(t14688: F, t92955: F, t4452: F, t92951: F, t14719: F, t25227: F, t2661: F, t14723: F, t25266: F, t4426: F, t1561: F, t93048: F) -> (F, F, F, F, F, F) {
    let t99021 = t92955 * t14688;
    let t99023 = t92951 * t4452;
    let t99026 = t2661 * t25227 * t14719;
    let t99029 = t2661 * t25227 * t14723;
    let t99033 = t25266 * t4426;
    let t99035 = t93048 * t1561;
    (t99021, t99023, t99026, t99029, t99033, t99035)
}
