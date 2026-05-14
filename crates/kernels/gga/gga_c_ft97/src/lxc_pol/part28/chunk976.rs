//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 976/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk976<F: Float>(t34817: F, t379: F, t2178: F, t7312: F, t106296: F, t106573: F, t106729: F, t106894: F, t120449: F, t12968: F, t13140: F, t139661: F, t139679: F, t139716: F, t139722: F, t1901: F, t2097: F, t2210: F, t23571: F, t26883: F, t26950: F, t26955: F, t26999: F, t27064: F, t27068: F, t27228: F, t33067: F, t33199: F, t3441: F, t3450: F, t3478: F, t3483: F, t35110: F, t35155: F, t50773: F, t51151: F, t5855: F, t5947: F, t5956: F, t64242: F, t7357: F, t7390: F, t9144: F, t925: F, t95789: F) -> (F, F) {
    let t147730 = t34817 * t379;
    let t147779 = t2178 * t7312;
    let t147788 = -2.0 / 27.0 * t1901 * t2097 * t7390 * t3441 - 4.0 / 3.0 * t1901 * t12968 * t106573 * t5947 - 4.0 / 3.0 * t1901 * t13140 * t120449 * t5956 + 2.0 / 3.0 * t1901 * t51151 * t147730 - 2.0 / 9.0 * t1901 * t9144 * t35155 * t379 - 4.0 / 9.0 * t1901 * t106894 * t27064 - 4.0 * t1901 * t26999 * t5855 * t26950 - 4.0 / 3.0 * t1901 * t12968 * t23571 * t26955 - t1901 * t9144 * t35110 * t379 / 9.0 + 4.0 / 9.0 * t139716 + t1901 * t2210 * t139661 * t925 / 9.0 + 4.0 / 9.0 * t139722 - 2.0 / 9.0 * t1901 * t95789 * t27228 - 4.0 / 9.0 * t1901 * t106296 * t27068 - 2.0 / 9.0 * t1901 * t50773 * t33067 + 4.0 * t1901 * t106729 * t7357 * t3450 - 4.0 / 3.0 * t1901 * t64242 * t33199 + 2.0 * t1901 * t26999 * t139679 * t3478 + 4.0 / 3.0 * t1901 * t12968 * t147779 * t3483 - 4.0 / 3.0 * t1901 * t12968 * t23571 * t26883;
    (t147730, t147788)
}
