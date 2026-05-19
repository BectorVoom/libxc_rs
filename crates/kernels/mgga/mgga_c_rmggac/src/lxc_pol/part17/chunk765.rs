//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 765/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk765<F: Float>(t35861: F, t321: F, t830: F, t262: F, t2068: F, t2067: F, t25529: F, t2079: F, t352: F, t333: F, t2073: F, t22: F, t4616: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35862 = F::cast_from(0.68297526743963945143e0_f64) * t35861;
    let t35875 = t830 * t321;
    let t35876 = t262 * t35875;
    let t35877 = t2068 * t35876;
    let t35879 = t25529 * t2067;
    let t35922 = t2079 * t262 * t830 * t352;
    let t35924 = t830 * t333;
    let t35925 = t262 * t35924;
    let t35926 = t2073 * t35925;
    let t35928 = t4616 * t22;
    (t35862, t35875, t35876, t35877, t35879, t35922, t35924, t35925, t35926, t35928)
}
