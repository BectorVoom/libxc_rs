//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1221/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1221<F: Float>(t18645: F, t18661: F, t18669: F, t18674: F, t18679: F, t18683: F, t18828: F, t18830: F, t18833: F, t18835: F, t18904: F, t10923: F, t10924: F, t13710: F, t13945: F, t13949: F, t18924: F, t18927: F, t18930: F, t18933: F, t18935: F, t18937: F) -> (F, F) {
    let t20431 = F::new(0.3529725e1) * t18835 + F::new(0.264729375e1) * t18828 - F::new(0.3529725e1) * t18830 - F::new(0.17648625e1) * t18833 - F::new(0.34431666666666666667e0) * t18674 + F::new(0.103295e1) * t18679 + F::new(0.11477222222222222222e0) * t18645 - F::new(0.34431666666666666667e0) * t18661 + F::new(0.17215833333333333333e0) * t18669 - F::new(0.516475e0) * t18683 + F::new(0.20839e0) * t18904;
    let t20452 = -F::new(0.62517e0) * t18924 + F::new(0.83356e0) * t18927 + F::new(0.20839e0) * t18930 - F::new(0.34731666666666666667e-1) * t18933 - t10923 - t10924 - F::new(0.13892666666666666667e0) * t18935 + F::new(0.69463333333333333333e-1) * t18937 - F::new(0.23154444444444444445e0) * t13945 - F::new(0.45908888888888888888e0) * t13710 + F::new(0.27785333333333333334e0) * t13949;
    (t20431, t20452)
}
