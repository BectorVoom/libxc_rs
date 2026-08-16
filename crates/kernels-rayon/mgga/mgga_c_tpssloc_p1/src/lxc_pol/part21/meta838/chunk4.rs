//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2994/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2994(t10390: f64, t18041: f64, t10422: f64, t18024: f64, t3070: f64, t13969: f64, t17733: f64, t3130: f64, t10214: f64, t1041: f64, t10883: f64, t10937: f64, t14080: f64, t14187: f64, t17596: f64, t17697: f64, t17712: f64, t17998: f64, t2960: f64, t3039: f64, t3041: f64, t3117: f64, t3121: f64, t43248: f64, t43253: f64, t4582: f64, t4585: f64, t4588: f64, t48496: f64, t50272: f64, t59751: f64, t61798: f64, t61855: f64, t61910: f64, t973: f64) -> f64 {
    let t62682 = t10390 * t18041;
    let t62687 = t3070 * t10422 * t18024;
    let t62704 = t3130 * t13969 * t17733;
    let t62722 = -t50272 / 324.0_f64 + t62682 / 1728.0_f64 - 5.0_f64 / 1296.0_f64 * t10937 * t17998 - t62687 / 864.0_f64 - t43248 / 972.0_f64 - t43253 - 7.0_f64 / 54.0_f64 * t973 * t10214 * t59751 - 2.0_f64 / 81.0_f64 * t2960 * t17596 - t3039 * t4582 * t17712 * t3121 / 3072.0_f64 + t10883 * t4582 * t17712 * t3041 / 3072.0_f64 + t62704 / 576.0_f64 + 5.0_f64 / 6912.0_f64 * t1041 * t4582 * t4588 * t61798 + 5.0_f64 / 2592.0_f64 * t3117 * t17697 + 5.0_f64 / 5184.0_f64 * t1041 * t4582 * t14187 * t61910 + 55.0_f64 / 15552.0_f64 * t1041 * t4582 * t48496 * t61855 + t14080 * t4585 / 108.0_f64;
    t62722
}
