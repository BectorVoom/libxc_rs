//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1354/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1354<F: Float>(t1169: F, t17085: F, t1179: F, t5155: F, t1719: F, t3383: F, t3386: F, t1749: F, t3520: F, t16868: F, t16712: F, t12297: F, t12299: F, t12301: F, t12303: F, t16706: F, t16727: F, t16748: F, t16871: F, t16876: F) -> (F, F, F, F, F) {
    let t17086 = t17085 * t1169;
    let t17089 = t5155 * t1179;
    let t17092 = t1719 * t3383;
    let t17094 = F::cast_from(2.0_f64) * t17092 * t3386;
    let t17097 = t1749 * t3520;
    let t17115 = F::cast_from(0.11038e0_f64) * t16868;
    let t17117 = F::cast_from(0.20128333333333333334e0_f64) * t16712;
    let t17126 = -t17115 + F::cast_from(0.82785e-1_f64) * t16871 - t17117 + F::cast_from(0.301925e0_f64) * t16748 + F::cast_from(0.13418888888888888889e0_f64) * t16706 + F::cast_from(0.91983333333333333334e-1_f64) * t16876 + F::cast_from(0.67094444444444444447e-1_f64) * t12299 + F::cast_from(0.26837777777777777778e0_f64) * t12297 - F::cast_from(0.20128333333333333334e0_f64) * t12301 - F::cast_from(0.10064166666666666667e0_f64) * t12303 - F::cast_from(0.40256666666666666666e0_f64) * t16727;
    (t17086, t17089, t17094, t17097, t17126)
}
