//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2986/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2986(t10403: f64, t10422: f64, t18035: f64, t10904: f64, t10937: f64, t13995: f64, t14033: f64, t14037: f64, t14085: f64, t14174: f64, t17734: f64, t17988: f64, t18021: f64, t18036: f64, t2960: f64, t3070: f64, t3071: f64, t3121: f64, t42505: f64, t43114: f64, t4590: f64, t4644: f64, t49972: f64, t49987: f64, t49989: f64, t49993: f64, t5681: f64) -> f64 {
    let t62418 = t10403 * t10422 * t18035;
    let t62427 = -t10904 * t17734 / 72.0_f64 - 5.0_f64 / 1152.0_f64 * t4644 * t14174 + 5.0_f64 / 3456.0_f64 * t14085 * t4590 - t49972 / 324.0_f64 - t43114 / 10368.0_f64 + t13995 * t14033 / 2304.0_f64 + 5.0_f64 / 6912.0_f64 * t13995 * t14037 - t3070 * t3071 * t5681 * t3121 / 2304.0_f64 - t42505 * t18036 / 216.0_f64 + t62418 / 1728.0_f64 - t10937 * t18021 / 432.0_f64 - t49987 / 216.0_f64 - t49989 / 216.0_f64 - t49993 / 3456.0_f64 - t2960 * t17988 / 9.0_f64;
    t62427
}
