//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 891/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk891(t12021: f64, t1385: f64, t8793: f64, t32147: f64, t539: f64, t1323: f64, t8788: f64, t31648: f64, t8800: f64, t3887: f64, t31662: f64, t1375: f64, t31609: f64, t31613: f64, t31646: f64, t31651: f64, t3758: f64, t3882: f64, t568: f64, t7194: f64, t7199: f64, t8794: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32161 = t12021 * t8793 * t1385;
    let t32164 = t539 * t32147;
    let t32168 = t1323 * t8788;
    let t32173 = 0.76763589786250567037e-1_f64 * t31648;
    let t32175 = t8800 * t1385;
    let t32176 = t3887 * t32175;
    let t32183 = 0.76763589786250567037e-1_f64 * t31662;
    let t32184 = -6.0_f64 * t1375 * t32161 + t32164 * t568 - 0.3289868133696452873e-1_f64 * t31609 - 0.3289868133696452873e-1_f64 * t31613 + t32168 * t568 + 4.0_f64 * t7194 * t7199 - 0.6579736267392905746e-1_f64 * t31646 + t32173 - 0.3289868133696452873e-1_f64 * t31651 + 2.0_f64 * t1375 * t32176 + 2.0_f64 * t3758 * t8794 + 2.0_f64 * t3882 * t8794 - t32183;
    (t32161, t32164, t32168, t32173, t32176, t32183, t32184)
}
