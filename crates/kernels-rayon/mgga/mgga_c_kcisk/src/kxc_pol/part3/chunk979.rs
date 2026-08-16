//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 979/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk979(t1474: f64, t979: f64, t4265: f64, t4279: f64, t12863: f64, t12874: f64, t13186: f64, t13201: f64, t13206: f64, t13278: f64, t13282: f64, t1429: f64, t1434: f64, t14405: f64, t14409: f64, t14434: f64, t1460: f64, t3560: f64, t3566: f64, t3588: f64, t3594: f64, t3620: f64, t4244: f64, t476: f64, t6256: f64, t6267: f64) -> f64 {
    let t14439 = t979 * t1474;
    let t14441 = t4265 * t4279;
    let t14443 = -0.79593333333333333333e-1_f64 * t14405 - t14409 + 0.371475e-1_f64 * t1460 * t3594 - 0.619125e-2_f64 * t476 * t13278 + 0.27860625e-1_f64 * t4244 * t1429 - 0.1857375e-1_f64 * t4244 * t1434 + 0.27860625e-1_f64 * t1460 * t3588 - 0.1857375e-1_f64 * t1460 * t3620 + 0.9286875e-2_f64 * t476 * t13282 - 0.371475e-1_f64 * t476 * t12874 + 0.139303125e-1_f64 * t1460 * t3560 - 0.139303125e-1_f64 * t6256 * t13201 + 0.139303125e-1_f64 * t6256 * t13206 - 0.232171875e-2_f64 * t476 * t13186 - 0.5572125e-1_f64 * t14434 * t3566 + 0.371475e-1_f64 * t6267 * t12863 + 0.17687407407407407407e-1_f64 * t14439 - 0.10612444444444444444e0_f64 * t14441;
    t14443
}
