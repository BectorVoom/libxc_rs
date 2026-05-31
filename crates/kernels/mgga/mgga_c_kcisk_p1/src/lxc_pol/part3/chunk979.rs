//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 979/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk979<F: Float>(t1474: F, t979: F, t4265: F, t4279: F, t12863: F, t12874: F, t13186: F, t13201: F, t13206: F, t13278: F, t13282: F, t1429: F, t1434: F, t14405: F, t14409: F, t14434: F, t1460: F, t3560: F, t3566: F, t3588: F, t3594: F, t3620: F, t4244: F, t476: F, t6256: F, t6267: F) -> F {
    let t14439 = t979 * t1474;
    let t14441 = t4265 * t4279;
    let t14443 = -F::cast_from(0.79593333333333333333e-1_f64) * t14405 - t14409 + F::cast_from(0.371475e-1_f64) * t1460 * t3594 - F::cast_from(0.619125e-2_f64) * t476 * t13278 + F::cast_from(0.27860625e-1_f64) * t4244 * t1429 - F::cast_from(0.1857375e-1_f64) * t4244 * t1434 + F::cast_from(0.27860625e-1_f64) * t1460 * t3588 - F::cast_from(0.1857375e-1_f64) * t1460 * t3620 + F::cast_from(0.9286875e-2_f64) * t476 * t13282 - F::cast_from(0.371475e-1_f64) * t476 * t12874 + F::cast_from(0.139303125e-1_f64) * t1460 * t3560 - F::cast_from(0.139303125e-1_f64) * t6256 * t13201 + F::cast_from(0.139303125e-1_f64) * t6256 * t13206 - F::cast_from(0.232171875e-2_f64) * t476 * t13186 - F::cast_from(0.5572125e-1_f64) * t14434 * t3566 + F::cast_from(0.371475e-1_f64) * t6267 * t12863 + F::cast_from(0.17687407407407407407e-1_f64) * t14439 - F::cast_from(0.10612444444444444444e0_f64) * t14441;
    t14443
}
