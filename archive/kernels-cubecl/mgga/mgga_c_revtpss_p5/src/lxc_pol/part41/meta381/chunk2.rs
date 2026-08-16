//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1261/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1261<F: Float>(t19172: F, t19253: F, t19293: F, t19334: F, t300: F, t6350: F, t999: F, t3269: F, t342: F, t6343: F, t11133: F, t11134: F, t15127: F, t15189: F, t15638: F, t15639: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18944: F, t18948: F) -> (F, F, F, F) {
    let t19337 = t300 * (t19172 + t19253 + t19293 + t19334);
    let t19341 = t6350 * t999;
    let t19342 = t3269 * t19341;
    let t19351 = t342 * t6343;
    let t19380 = -t11133 - F::cast_from(0.65851851851851851853e-2_f64) * t11134 - F::cast_from(0.13170370370370370371e-1_f64) * t15189 + F::cast_from(0.65851851851851851853e-2_f64) * t15127 - t15638 + t15639 + F::cast_from(0.32925925925925925927e-2_f64) * t18919 - F::cast_from(0.16462962962962962963e-1_f64) * t18906 + F::cast_from(0.59266666666666666668e-1_f64) * t18911 - F::cast_from(0.19755555555555555556e-1_f64) * t18915 - F::cast_from(0.9877777777777777778e-2_f64) * t18924 - F::cast_from(0.88900000000000000002e-1_f64) * t18928 + F::cast_from(0.59266666666666666668e-1_f64) * t18932 + F::cast_from(0.4938888888888888889e-2_f64) * t18934 - F::cast_from(0.9877777777777777778e-2_f64) * t18939 + F::cast_from(0.29633333333333333334e-1_f64) * t18944 - F::cast_from(0.14816666666666666667e-1_f64) * t18948;
    (t19337, t19342, t19351, t19380)
}
