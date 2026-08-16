//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2813/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2813(t17100: f64, t225: f64, t10110: f64, t13029: f64, t13042: f64, t13065: f64, t13072: f64, t13461: f64, t1519: f64, t1528: f64, t16804: f64, t17057: f64, t17070: f64, t17092: f64, t259: f64, t2591: f64, t2713: f64, t2720: f64, t2742: f64, t4142: f64, t4147: f64, t4265: f64, t4273: f64, t4301: f64, t47568: f64, t5631: f64, t5636: f64, t5637: f64, t5658: f64, t852: f64, t855: f64, t866: f64, t9590: f64, t9593: f64) -> f64 {
    let t59466 = t17100 * t225;
    let t59475 = t2591 * t5631 * t259 + 2.0_f64 * t13029 * t1519 * t259 - 4.0_f64 * t47568 * t1528 + 8.0_f64 * t4147 * t13072 + 8.0_f64 * t13065 * t4273 - 2.0_f64 * t4147 * t13461 + 2.0_f64 * t16804 * t852 * t259 + 4.0_f64 * t17092 * t2720 + 4.0_f64 * t4142 * t4265 * t259 - 4.0_f64 * t13065 * t4301 + 8.0_f64 * t2713 * t17070 - 2.0_f64 * t9593 * t5658 - 4.0_f64 * t13042 * t4301 + 4.0_f64 * t2713 * t17057 - 2.0_f64 * t59466 * t866 - 6.0_f64 * t855 * t10110 * t5636 * t2742 + 2.0_f64 * t9590 * t5637;
    t59475
}
