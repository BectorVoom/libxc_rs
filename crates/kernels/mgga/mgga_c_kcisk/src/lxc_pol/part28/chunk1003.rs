//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1003/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1003<F: Float>(t696: F, t7718: F, t5136: F, t11605: F, t11613: F, t11633: F, t11635: F, t158: F, t165: F, t173: F, t1809: F, t1850: F, t22591: F, t23234: F, t23236: F, t23238: F, t23240: F, t23243: F, t23246: F, t23249: F, t23251: F, t23253: F, t23255: F, t23259: F) -> (F,) {
    let t23261 = t696 * t7718;
    let t23263 = t5136 * t7718;
    let t23269 = 0.15684083333333333333e-4 * t23234 + 0.23526125e-4 * t23236 - 0.13208333333333333333e-2 * t23238 + 0.7925e-3 * t165 * t23240 + 0.50413125e-5 * t173 * t23243 - 0.3513e-2 * t158 * t23246 + 0.4684e-2 * t23249 - 0.15613333333333333333e-2 * t23251 - 0.9368e-2 * t23253 + 0.26416666666666666667e-2 * t23255 + 0.23911438650126355246e-1 * t11605 - 0.31077233446777841256e-3 * t11613 + 0.23911438650126355246e-1 * t23259 - 0.11955719325063177623e-1 * t23261 + 0.10359077815592613752e-3 * t23263 + 0.11955719325063177623e-1 * t1809 * t22591 - 0.5179538907796306876e-4 * t1850 * t22591 + t11633 - t11635;
    (t23269,)
}
