//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1081/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1081(t2173: f64, t2175: f64, t26728: f64, t26732: f64, t26736: f64, t26739: f64, t26742: f64, t26745: f64, t26748: f64, t26751: f64, t26755: f64, t26758: f64, t26764: f64, t26767: f64, t26774: f64, t7687: f64, t7690: f64, t7693: f64, t7703: f64, t7706: f64) -> f64 {
    let t26776 = 0.13901041666666666667e-2_f64 * t7687 * t7693 + 0.18550940104166666667e-3_f64 * t26728 * t7693 + 0.92754700520833333333e-4_f64 * t7690 * t26732 + 0.69505208333333333333e-3_f64 * t2173 * t26736 - 0.4946917361111111111e-3_f64 * t26739 * t7693 - 0.67960648148148148147e-2_f64 * t26742 * t2175 + 0.12356481481481481482e-2_f64 * t26745 - 0.46336805555555555556e-3_f64 * t26748 * t7706 + 0.22109259259259259258e-2_f64 * t26751 + 0.33163888888888888888e-2_f64 * t26755 - 0.15445601851851851852e-3_f64 * t26758 - 0.33163888888888888888e-2_f64 * t26764 - 0.23168402777777777778e-3_f64 * t7703 * t26767 + 0.69505208333333333333e-3_f64 * t2173 * t26732 + 0.49745833333333333332e-2_f64 * t26774;
    t26776
}
