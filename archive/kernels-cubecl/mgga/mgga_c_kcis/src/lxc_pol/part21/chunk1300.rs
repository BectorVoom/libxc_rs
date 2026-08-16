//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1300/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1300<F: Float>(t359: F, t92807: F, t14386: F, t4554: F, t1709: F, t330: F, t26806: F, t93426: F, t7690: F, t7703: F, t93425: F, t93592: F, t95691: F, t95832: F, t95884: F, t95887: F, t95892: F, t95895: F, t95898: F, t95903: F, t95906: F, t95909: F) -> (F, F, F, F, F) {
    let t95911 = t92807 * t359;
    let t95913 = t4554 * t95911 * t14386;
    let t95915 = t1709 * t330;
    let t95917 = t93426 * t95915 * t26806;
    let t95920 = F::cast_from(0.27802083333333333334e-2_f64) * t7703 * t95691 - F::cast_from(0.16581944444444444444e-2_f64) * t95884 + F::cast_from(0.88437037037037037034e-2_f64) * t95887 - t95892 + F::cast_from(0.33163888888888888888e-2_f64) * t95895 + F::cast_from(0.92754700520833333333e-4_f64) * t7690 * t95898 - F::cast_from(0.12367293402777777778e-3_f64) * t93425 * t95832 - F::cast_from(0.22109259259259259258e-2_f64) * t95903 + F::cast_from(0.88437037037037037034e-2_f64) * t95906 + F::cast_from(0.33163888888888888888e-2_f64) * t95909 - F::cast_from(0.55273148148148148146e-2_f64) * t95913 - F::cast_from(0.46336805555555555556e-3_f64) * t93592 * t95917;
    (t95911, t95913, t95915, t95917, t95920)
}
