//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1260/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1260(t10997: f64, t3275: f64, t43979: f64, t2333: f64, t3016: f64, t795: f64, t3262: f64, t3263: f64, t1065: f64, t3270: f64, t10667: f64, t38337: f64, t38339: f64, t38342: f64, t38347: f64, t38350: f64, t38356: f64, t38359: f64, t38363: f64, t43968: f64, t43971: f64, t43974: f64, t43976: f64, t43978: f64) -> (f64, f64, f64, f64) {
    let t43982 = 45.0_f64 / 32.0_f64 * t3275 * t10997 * t43979;
    let t43983 = t2333 * t3016;
    let t43984 = t43983 * t795;
    let t43987 = 3.0_f64 / 4.0_f64 * t3262 * t3263 * t43984;
    let t43988 = t1065 * t3016;
    let t43989 = t3270 * t43988;
    let t43991 = 3.0_f64 / 4.0_f64 * t10667 * t43989;
    let t43992 = t38337 + 0.81300399444200075504e-3_f64 * t38339 - t38342 + t38347 - t38350 - 0.19211284388664477842e-2_f64 * t38356 + 0.30487649791575028314e-3_f64 * t38359 + t38363 - t43968 + t43971 + t43974 - t43976 + t43978 - t43982 + t43987 - t43991;
    (t43982, t43987, t43991, t43992)
}
