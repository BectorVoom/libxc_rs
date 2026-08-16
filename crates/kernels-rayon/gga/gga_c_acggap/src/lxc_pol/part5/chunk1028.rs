//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1028/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1028(t13083: f64, t2450: f64, t4465: f64, t14056: f64, t4732: f64, t4396: f64, t4456: f64, t157: f64, t3101: f64, t1163: f64, t1165: f64, t1532: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17550 = t2450 * t13083;
    let t17551 = t17550 * t4465;
    let t17557 = t14056 * t4732;
    let t17567 = t4396 * t4456;
    let t17581 = t157 * t3101;
    let t17584 = t1163 * t1165 * t1532 * t17581;
    (t17550, t17551, t17557, t17567, t17581, t17584)
}
