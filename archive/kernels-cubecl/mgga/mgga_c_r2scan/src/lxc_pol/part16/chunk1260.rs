//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1260/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1260<F: Float>(t10997: F, t3275: F, t43979: F, t2333: F, t3016: F, t795: F, t3262: F, t3263: F, t1065: F, t3270: F, t10667: F, t38337: F, t38339: F, t38342: F, t38347: F, t38350: F, t38356: F, t38359: F, t38363: F, t43968: F, t43971: F, t43974: F, t43976: F, t43978: F) -> (F, F, F, F) {
    let t43982 = F::cast_from(45.0_f64) / F::cast_from(32.0_f64) * t3275 * t10997 * t43979;
    let t43983 = t2333 * t3016;
    let t43984 = t43983 * t795;
    let t43987 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t3262 * t3263 * t43984;
    let t43988 = t1065 * t3016;
    let t43989 = t3270 * t43988;
    let t43991 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t10667 * t43989;
    let t43992 = t38337 + F::cast_from(0.81300399444200075504e-3_f64) * t38339 - t38342 + t38347 - t38350 - F::cast_from(0.19211284388664477842e-2_f64) * t38356 + F::cast_from(0.30487649791575028314e-3_f64) * t38359 + t38363 - t43968 + t43971 + t43974 - t43976 + t43978 - t43982 + t43987 - t43991;
    (t43982, t43987, t43991, t43992)
}
