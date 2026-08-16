//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1300/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1300(t359: f64, t92807: f64, t14386: f64, t4554: f64, t1709: f64, t330: f64, t26806: f64, t93426: f64, t7690: f64, t7703: f64, t93425: f64, t93592: f64, t95691: f64, t95832: f64, t95884: f64, t95887: f64, t95892: f64, t95895: f64, t95898: f64, t95903: f64, t95906: f64, t95909: f64) -> (f64, f64, f64, f64, f64) {
    let t95911 = t92807 * t359;
    let t95913 = t4554 * t95911 * t14386;
    let t95915 = t1709 * t330;
    let t95917 = t93426 * t95915 * t26806;
    let t95920 = 0.27802083333333333334e-2_f64 * t7703 * t95691 - 0.16581944444444444444e-2_f64 * t95884 + 0.88437037037037037034e-2_f64 * t95887 - t95892 + 0.33163888888888888888e-2_f64 * t95895 + 0.92754700520833333333e-4_f64 * t7690 * t95898 - 0.12367293402777777778e-3_f64 * t93425 * t95832 - 0.22109259259259259258e-2_f64 * t95903 + 0.88437037037037037034e-2_f64 * t95906 + 0.33163888888888888888e-2_f64 * t95909 - 0.55273148148148148146e-2_f64 * t95913 - 0.46336805555555555556e-3_f64 * t93592 * t95917;
    (t95911, t95913, t95915, t95917, t95920)
}
