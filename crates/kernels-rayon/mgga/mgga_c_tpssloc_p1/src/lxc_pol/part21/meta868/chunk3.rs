//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3179/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3179(t15568: f64, t5064: f64, t1174: f64, t18206: f64, t44562: f64, t1227: f64, t13969: f64, t18958: f64, t11665: f64, t11668: f64, t11678: f64, t11692: f64, t15569: f64, t15591: f64, t15714: f64, t18342: f64, t18387: f64, t3490: f64, t3494: f64, t3509: f64, t3516: f64, t3577: f64, t3578: f64, t3580: f64, t44621: f64, t4950: f64, t5014: f64, t52751: f64, t52758: f64, t52773: f64, t53322: f64, t5971: f64, t5975: f64, t63420: f64) -> f64 {
    let t65884 = t5064 * t15568;
    let t65914 = t1174 * t44562 * t18206;
    let t65920 = t1227 * t13969 * t18958;
    let t65925 = t65884 * t3580 / 216.0_f64 - t11678 * t3578 * t5975 * t3509 / 1152.0_f64 + t11692 * t3578 * t5975 * t3516 / 2304.0_f64 - t11665 * t18387 / 1152.0_f64 - t3577 * t3578 * t5975 * t3494 / 2304.0_f64 - 5.0_f64 / 1296.0_f64 * t15569 * t15714 + t15591 * t5014 / 768.0_f64 - 7.0_f64 / 972.0_f64 * t52751 - t53322 * t4950 / 1152.0_f64 + t52758 / 5184.0_f64 + 5.0_f64 / 6912.0_f64 * t11678 * t11668 * t5971 * t3509 - 7.0_f64 / 972.0_f64 * t65914 + 35.0_f64 / 972.0_f64 * t1174 * t44621 * t63420 - t65920 / 1728.0_f64 + 5.0_f64 / 3456.0_f64 * t3490 * t18342 - t52773 / 216.0_f64;
    t65925
}
