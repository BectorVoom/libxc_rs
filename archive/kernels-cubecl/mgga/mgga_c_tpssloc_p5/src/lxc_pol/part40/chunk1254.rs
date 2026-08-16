//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1254/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1254<F: Float>(t1367: F, t19631: F, t820: F, t16336: F, t1831: F, t12308: F, t12325: F, t12330: F, t12335: F, t1363: F, t1369: F, t16321: F, t16346: F, t16350: F, t16354: F, t19904: F, t19915: F, t19917: F, t19921: F, t19926: F, t3778: F, t3783: F, t5240: F, t5310: F, t5314: F, t6422: F, t6427: F, t6431: F) -> F {
    let t19930 = t1367 * t820 * t19631;
    let t19933 = t16336 * t1831;
    let t19939 = -F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t12308 - t16346 + F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t16350 + t16354 + F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t12325 - t12330 - t12335 - t19904 * t1369 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t3783 * t6427 - t3783 * t6431 / F::cast_from(768.0_f64) - t3778 * t6422 / F::cast_from(3072.0_f64) + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t5240 * t5310 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t19915 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t19917 - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t1363 * t19921 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t1363 * t19926 - t1363 * t19930 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t19933 - t16321 * t1831 / F::cast_from(384.0_f64) - t5240 * t5314 / F::cast_from(384.0_f64);
    t19939
}
