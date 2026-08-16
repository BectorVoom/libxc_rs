//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2989/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2989(t1041: f64, t13969: f64, t17642: f64, t17906: f64, t3117: f64, t10390: f64, t10403: f64, t10413: f64, t10965: f64, t1618: f64, t17920: f64, t17976: f64, t3041: f64, t3048: f64, t3071: f64, t3132: f64, t42511: f64, t43155: f64, t43157: f64, t43161: f64, t4596: f64, t50062: f64, t50077: f64, t50302: f64, t50445: f64, t5681: f64, t5900: f64, t5909: f64) -> f64 {
    let t62515 = t1041 * t13969 * t17642;
    let t62534 = t3117 * t17906;
    let t62544 = 5.0_f64 / 10368.0_f64 * t62515 - 11.0_f64 / 486.0_f64 * t43155 - 5.0_f64 / 243.0_f64 * t43157 - t10403 * t3071 * t5681 * t3132 / 1152.0_f64 + t10413 * t3071 * t5681 * t3041 / 2304.0_f64 + t42511 * t5909 / 2304.0_f64 - t43161 / 13824.0_f64 - t10965 * t5900 / 2304.0_f64 - t50445 * t1618 / 144.0_f64 - t62534 / 1728.0_f64 + t50062 / 576.0_f64 - t50302 * t4596 / 72.0_f64 + t3048 * t17976 / 108.0_f64 + 2.0_f64 / 243.0_f64 * t50077 + 5.0_f64 / 3456.0_f64 * t10390 * t17920;
    t62544
}
