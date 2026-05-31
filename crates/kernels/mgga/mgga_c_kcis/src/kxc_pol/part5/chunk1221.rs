//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1221/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1221<F: Float>(t18645: F, t18661: F, t18669: F, t18674: F, t18679: F, t18683: F, t18828: F, t18830: F, t18833: F, t18835: F, t18904: F, t10923: F, t10924: F, t13710: F, t13945: F, t13949: F, t18924: F, t18927: F, t18930: F, t18933: F, t18935: F, t18937: F) -> (F, F) {
    let t20431 = F::cast_from(0.3529725e1_f64) * t18835 + F::cast_from(0.264729375e1_f64) * t18828 - F::cast_from(0.3529725e1_f64) * t18830 - F::cast_from(0.17648625e1_f64) * t18833 - F::cast_from(0.34431666666666666667e0_f64) * t18674 + F::cast_from(0.103295e1_f64) * t18679 + F::cast_from(0.11477222222222222222e0_f64) * t18645 - F::cast_from(0.34431666666666666667e0_f64) * t18661 + F::cast_from(0.17215833333333333333e0_f64) * t18669 - F::cast_from(0.516475e0_f64) * t18683 + F::cast_from(0.20839e0_f64) * t18904;
    let t20452 = -F::cast_from(0.62517e0_f64) * t18924 + F::cast_from(0.83356e0_f64) * t18927 + F::cast_from(0.20839e0_f64) * t18930 - F::cast_from(0.34731666666666666667e-1_f64) * t18933 - t10923 - t10924 - F::cast_from(0.13892666666666666667e0_f64) * t18935 + F::cast_from(0.69463333333333333333e-1_f64) * t18937 - F::cast_from(0.23154444444444444445e0_f64) * t13945 - F::cast_from(0.45908888888888888888e0_f64) * t13710 + F::cast_from(0.27785333333333333334e0_f64) * t13949;
    (t20431, t20452)
}
