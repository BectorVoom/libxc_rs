//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1214/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1214<F: Float>(t1095: F, t5871: F, t1937: F, t2793: F, t1083: F, t1899: F, t5581: F, t20716: F, t17351: F, t17354: F, t17357: F, t17728: F, t20705: F, t20719: F, t20745: F, t228: F) -> (F, F, F, F) {
    let t21173 = t5871 * t1095;
    let t21179 = t2793 * t1937;
    let t21184 = t1899 * t1083;
    let t21186 = F::cast_from(18.0_f64) * t21184 * t5581;
    let t21191 = F::cast_from(0.71233333333333333332e-1_f64) * t20716;
    let t21196 = F::cast_from(0.621814e-1_f64) * (t17728 - F::cast_from(0.16621111111111111111e0_f64) * t17351 + F::cast_from(0.71233333333333333332e-1_f64) * t17354 - F::cast_from(0.17808333333333333333e-1_f64) * t17357 - F::cast_from(0.55403703703703703703e-1_f64) * t20705 + t21191 - F::cast_from(0.53424999999999999999e-1_f64) * t20719 + F::cast_from(0.53425e-1_f64) * t20745) * t228;
    (t21173, t21179, t21186, t21196)
}
