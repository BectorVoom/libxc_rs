//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 873/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk873<F: Float>(t12863: F, t12874: F, t13186: F, t13201: F, t13206: F, t13278: F, t13282: F, t1429: F, t1434: F, t14405: F, t14409: F, t14434: F, t14439: F, t14441: F, t1460: F, t3560: F, t3566: F, t3588: F, t3594: F, t3620: F, t4244: F, t476: F, t6256: F, t6267: F) -> (F,) {
    let t14443 = -0.79593333333333333333e-1 * t14405 - t14409 + 0.371475e-1 * t1460 * t3594 - 0.619125e-2 * t476 * t13278 + 0.27860625e-1 * t4244 * t1429 - 0.1857375e-1 * t4244 * t1434 + 0.27860625e-1 * t1460 * t3588 - 0.1857375e-1 * t1460 * t3620 + 0.9286875e-2 * t476 * t13282 - 0.371475e-1 * t476 * t12874 + 0.139303125e-1 * t1460 * t3560 - 0.139303125e-1 * t6256 * t13201 + 0.139303125e-1 * t6256 * t13206 - 0.232171875e-2 * t476 * t13186 - 0.5572125e-1 * t14434 * t3566 + 0.371475e-1 * t6267 * t12863 + 0.17687407407407407407e-1 * t14439 - 0.10612444444444444444e0 * t14441;
    (t14443,)
}
