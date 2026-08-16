//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 966/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk966<F: Float>(t22227: F, t22242: F, t475: F, t1214: F, t248: F, t21510: F, t4972: F, t4582: F, t11834: F, t1213: F, t1227: F, t15717: F, t15719: F, t15727: F, t15731: F, t15735: F, t1737: F, t1748: F, t18978: F, t18980: F, t18987: F, t19026: F, t19033: F, t19041: F, t19080: F, t22208: F, t22214: F, t22218: F, t5024: F, t6203: F, t6211: F) -> (F, F) {
    let t22243 = t22227 + t22242;
    let t22244 = t22243 * t475;
    let t22246 = t248 * t1214 * t22244;
    let t22257 = t4972 * t21510;
    let t22258 = t4582 * t22257;
    let t22267 = -t18978 / F::cast_from(144.0_f64) - t18980 / F::cast_from(1152.0_f64) + t18987 / F::cast_from(216.0_f64) - F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t1227 * t22208 + t5024 * t6211 / F::cast_from(144.0_f64) - t1227 * t22214 / F::cast_from(4608.0_f64) - t1227 * t22218 / F::cast_from(768.0_f64) + t11834 + t1213 * t22246 / F::cast_from(3072.0_f64) + t15717 / F::cast_from(864.0_f64) - t15719 / F::cast_from(4608.0_f64) + t15727 / F::cast_from(54.0_f64) - t15731 / F::cast_from(4608.0_f64) + t15735 / F::cast_from(6912.0_f64) - t19041 / F::cast_from(2304.0_f64) - F::cast_from(5.0_f64) / F::cast_from(864.0_f64) * t5024 * t6203 - t1227 * t22258 / F::cast_from(768.0_f64) + F::cast_from(19.0_f64) / F::cast_from(576.0_f64) * t19026 * t1737 - F::cast_from(19.0_f64) / F::cast_from(864.0_f64) * t19033 * t1748 - t19080 * t1737 / F::cast_from(96.0_f64);
    (t22243, t22267)
}
