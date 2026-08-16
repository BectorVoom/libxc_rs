//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2916/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2916(t17349: f64, t2888: f64, t13520: f64, t14422: f64, t10740: f64, t10747: f64, t10765: f64, t10825: f64, t14263: f64, t14337: f64, t14450: f64, t14460: f64, t17350: f64, t17443: f64, t17446: f64, t17451: f64, t17454: f64, t17493: f64, t17538: f64, t17541: f64, t17544: f64, t17548: f64, t17551: f64, t17555: f64, t2861: f64, t2886: f64, t41984: f64, t42128: f64, t42149: f64, t4454: f64, t4476: f64, t49096: f64, t49411: f64, t60360: f64, t931: f64, t932: f64) -> (f64, f64) {
    let t60775 = t17349 * t2888;
    let t60787 = 12.0_f64 * t13520 * t14422;
    let t60806 = -8.0_f64 * t10740 * t17538 - 0.38596750796862084162e3_f64 * t41984 * t17541 - 4.0_f64 * t10740 * t17544 + 0.64327917994770140268e2_f64 * t10765 * t17548 - 4.0_f64 * t2861 * t17350 * t931 + 0.64327917994770140268e2_f64 * t2886 * t60775 * t931 + 0.12865583598954028054e3_f64 * t10765 * t17551 + 0.4138081033541872024e4_f64 * t42149 * t17555 - 4.0_f64 * t2861 * t60360 * t932 - t60787 - 0.46785788981077169656e1_f64 * t49096 * t4454 + 0.69263436422725855034e2_f64 * t49411 * t4476 - 0.46785788981077169656e1_f64 * t14263 * t14450 + 0.69263436422725855034e2_f64 * t14337 * t14460 + 0.70178683471615754484e1_f64 * t10825 * t17443 - 0.46785788981077169656e1_f64 * t10747 * t17446 - 0.2077903092681775651e3_f64 * t42128 * t17451 - 0.23392894490538584828e1_f64 * t10747 * t17454 + 0.34631718211362927518e2_f64 * t10825 * t17493;
    (t60787, t60806)
}
