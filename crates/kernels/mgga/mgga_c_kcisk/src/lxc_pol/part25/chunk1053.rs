//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1053/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1053<F: Float>(t17076: F, t17086: F, t5439: F, t7644: F, t2028: F, t11652: F, t11986: F, t17084: F, t17093: F, t17096: F, t17100: F, t17104: F, t17109: F, t17114: F, t17117: F, t17120: F, t17124: F, t17129: F, t17135: F, t18772: F, t18776: F, t1994: F, t5348: F, t5445: F, t7553: F) -> (F, F, F) {
    let t18785 = 0.15476481481481481481e-2 * t17076;
    let t18787 = 0.10317654320987654321e-2 * t17086;
    let t18792 = t7644 * t5439;
    let t18793 = t18792 * t2028;
    let t18810 = t18785 + 0.19345601851851851852e-2 * t17084 + t18787 - 0.30952962962962962962e-2 * t17093 + 0.11607361111111111111e-2 * t17096 - 0.30952962962962962962e-2 * t17100 - 0.51588271604938271603e-2 * t17104 + 0.148996e0 * t5445 * t18793 + 0.11607361111111111111e-2 * t11652 + 0.61905925925925925924e-2 * t17109 + 0.386e0 * t5348 * t7553 - 0.11607361111111111111e-2 * t17114 - 0.77382407407407407406e-3 * t17117 - 0.61905925925925925924e-2 * t17120 + 0.193e0 * t1994 * t18772 - 0.43134342e-1 * t11986 * t18776 + 0.61905925925925925924e-2 * t17124 + 0.23214722222222222222e-2 * t17129 + 0.51588271604938271604e-3 * t17135;
    (t18792, t18793, t18810)
}
