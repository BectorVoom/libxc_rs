//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1318/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1318<F: Float>(t2932: F, t31927: F, t9307: F, t31910: F, t9301: F, t15251: F, t15253: F, t2677: F, t3934: F, t933: F, t31917: F, t9315: F, t111326: F, t9304: F, t110972: F, t110975: F, t110978: F, t111013: F, t111016: F) -> (F,) {
    let t111368 = t2932 * t31927 * t9307;
    let t111375 = t9301 * t31910;
    let t111380 = t2677 * t3934 * t15251 * t933 * t15253;
    let t111382 = t9301 * t31917;
    let t111384 = t9315 * t31910;
    let t111386 = t9304 * t111326;
    let t111388 = -0.56291666666666666668e-1 * t111368 + 0.79593333333333333331e-1 * t110972 - 0.92858888888888888885e-1 * t110975 - 0.29847499999999999999e-1 * t110978 - 0.39796666666666666665e-1 * t111013 - 0.10317654320987654321e0 * t111016 - 0.62500000000000000002e-1 * t111375 + 0.62500000000000000002e-1 * t111380 - 0.62500000000000000002e-1 * t111382 + 0.14583333333333333334e0 * t111384 + 0.44229166666666666667e-1 * t111386;
    (t111388,)
}
