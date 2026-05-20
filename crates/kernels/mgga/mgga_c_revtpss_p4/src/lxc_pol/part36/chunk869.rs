//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 869/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk869<F: Float>(t1412: F, t1882: F, t2470: F, t5721: F, t3915: F, t2435: F, t5600: F, t1426: F, t1893: F, t786: F, t136: F, t1903: F) -> (F, F, F, F, F, F) {
    let t14045 = t1412 * t1882;
    let t14090 = t5721 * t2470;
    let t14091 = t3915 * t14090;
    let t14097 = t2435 * t5600;
    let t14099 = t1893 * t1426;
    let t14100 = t786 * t14099;
    let t14103 = t1903 * t136;
    (t14045, t14090, t14091, t14097, t14100, t14103)
}
