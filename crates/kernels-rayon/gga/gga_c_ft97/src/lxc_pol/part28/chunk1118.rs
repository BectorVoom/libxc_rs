//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1118/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1118(t34817: f64, t379: f64, t2178: f64, t7312: f64, t106296: f64, t106573: f64, t106729: f64, t106894: f64, t120449: f64, t12968: f64, t13140: f64, t139661: f64, t139679: f64, t139716: f64, t139722: f64, t1901: f64, t2097: f64, t2210: f64, t23571: f64, t26883: f64, t26950: f64, t26955: f64, t26999: f64, t27064: f64, t27068: f64, t27228: f64, t33067: f64, t33199: f64, t3441: f64, t3450: f64, t3478: f64, t3483: f64, t35110: f64, t35155: f64, t50773: f64, t51151: f64, t5855: f64, t5947: f64, t5956: f64, t64242: f64, t7357: f64, t7390: f64, t9144: f64, t925: f64, t95789: f64) -> (f64, f64) {
    let t147730 = t34817 * t379;
    let t147779 = t2178 * t7312;
    let t147788 = -2.0_f64 / 27.0_f64 * t1901 * t2097 * t7390 * t3441 - 4.0_f64 / 3.0_f64 * t1901 * t12968 * t106573 * t5947 - 4.0_f64 / 3.0_f64 * t1901 * t13140 * t120449 * t5956 + 2.0_f64 / 3.0_f64 * t1901 * t51151 * t147730 - 2.0_f64 / 9.0_f64 * t1901 * t9144 * t35155 * t379 - 4.0_f64 / 9.0_f64 * t1901 * t106894 * t27064 - 4.0_f64 * t1901 * t26999 * t5855 * t26950 - 4.0_f64 / 3.0_f64 * t1901 * t12968 * t23571 * t26955 - t1901 * t9144 * t35110 * t379 / 9.0_f64 + 4.0_f64 / 9.0_f64 * t139716 + t1901 * t2210 * t139661 * t925 / 9.0_f64 + 4.0_f64 / 9.0_f64 * t139722 - 2.0_f64 / 9.0_f64 * t1901 * t95789 * t27228 - 4.0_f64 / 9.0_f64 * t1901 * t106296 * t27068 - 2.0_f64 / 9.0_f64 * t1901 * t50773 * t33067 + 4.0_f64 * t1901 * t106729 * t7357 * t3450 - 4.0_f64 / 3.0_f64 * t1901 * t64242 * t33199 + 2.0_f64 * t1901 * t26999 * t139679 * t3478 + 4.0_f64 / 3.0_f64 * t1901 * t12968 * t147779 * t3483 - 4.0_f64 / 3.0_f64 * t1901 * t12968 * t23571 * t26883;
    (t147730, t147788)
}
