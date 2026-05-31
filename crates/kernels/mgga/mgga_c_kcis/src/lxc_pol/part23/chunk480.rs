//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 480/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk480<F: Float>(t1307: F, t1380: F, t3984: F, t1444: F, t498: F, t2642: F, t1370: F, t1371: F, t2645: F, t1376: F, t497: F) -> (F, F, F, F, F, F, F) {
    let t3985 = t1307 * t1380;
    let t3986 = t3984 * t3985;
    let t3989 = t498 * t1444;
    let t3990 = t3989 * t2642;
    let t3991 = t1370 * t3990;
    let t3994 = t1371 * t2645;
    let t3995 = t1370 * t3994;
    let t3999 = F::cast_from(1.0_f64) / t1376 / t497;
    (t3985, t3986, t3990, t3991, t3994, t3995, t3999)
}
