//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1669/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1669(t1367: f64, t19631: f64, t820: f64, t16336: f64, t1831: f64, t12308: f64, t12325: f64, t12330: f64, t12335: f64, t1363: f64, t1369: f64, t16321: f64, t16346: f64, t16350: f64, t16354: f64, t19904: f64, t19915: f64, t19917: f64, t19921: f64, t19926: f64, t3778: f64, t3783: f64, t5240: f64, t5310: f64, t5314: f64, t6422: f64, t6427: f64, t6431: f64) -> (f64, f64) {
    let t19930 = t1367 * t820 * t19631;
    let t19933 = t16336 * t1831;
    let t19939 = -35.0_f64 / 216.0_f64 * t12308 - t16346 + 119.0_f64 / 6912.0_f64 * t16350 + t16354 + 119.0_f64 / 13824.0_f64 * t12325 - t12330 - t12335 - t19904 * t1369 / 768.0_f64 + 5.0_f64 / 768.0_f64 * t3783 * t6427 - t3783 * t6431 / 768.0_f64 - t3778 * t6422 / 3072.0_f64 + 5.0_f64 / 384.0_f64 * t5240 * t5310 + 7.0_f64 / 4608.0_f64 * t19915 + 7.0_f64 / 4608.0_f64 * t19917 - 5.0_f64 / 128.0_f64 * t1363 * t19921 + 5.0_f64 / 384.0_f64 * t1363 * t19926 - t1363 * t19930 / 768.0_f64 + 7.0_f64 / 576.0_f64 * t19933 - t16321 * t1831 / 384.0_f64 - t5240 * t5314 / 384.0_f64;
    (t19930, t19939)
}
