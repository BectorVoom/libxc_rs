//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1967/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1967<F: Float>(t1353: F, t6781: F, t30122: F, t1450: F, t21969: F, t1518: F, t4245: F, t1501: F, t4292: F, t1448: F, t21881: F, t93: F) -> (F, F, F, F, F, F, F) {
    let t109100 = t6781 * t1353;
    let t109104 = t30122 * t1353;
    let t109118 = t1450 * t21969;
    let t109150 = t4245 * t1518;
    let t109153 = t1501 * t4292;
    let t109199 = t30122 * t1448;
    let t109242 = t93 * t21881;
    (t109100, t109104, t109118, t109150, t109153, t109199, t109242)
}
