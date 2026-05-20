//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1465/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1465<F: Float>(t136: F, t1903: F, t2457: F, t9674: F, t10175: F, t5722: F, t122: F, t5721: F, t3916: F, t9680: F, t1437: F, t1882: F) -> (F, F, F, F, F, F, F) {
    let t14103 = t1903 * t136;
    let t14104 = t14103 * t2457;
    let t14105 = t9674 * t14104;
    let t14108 = F::cast_from(0.19514881078765566038e-1_f64) * t10175 * t5722;
    let t14109 = t5721 * t122;
    let t14110 = t14109 * t3916;
    let t14111 = t9680 * t14110;
    let t14113 = t1437 * t1882;
    (t14104, t14105, t14108, t14109, t14110, t14111, t14113)
}
