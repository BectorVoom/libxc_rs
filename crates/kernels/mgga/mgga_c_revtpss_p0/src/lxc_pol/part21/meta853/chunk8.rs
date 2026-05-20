//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3220/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3220<F: Float>(t13126: F, t1770: F, t1234: F, t12621: F, t12690: F, t12732: F, t1285: F, t1287: F, t13108: F, t13121: F, t13130: F, t13133: F, t16750: F, t17821: F, t17861: F, t1811: F, t1825: F, t3568: F, t3670: F, t3755: F, t3759: F, t3778: F, t5245: F, t5326: F, t5486: F, t57200: F, t57498: F) -> F {
    let t59550 = t1770 * t13126;
    let t59579 = F::cast_from(0.65854491829355115987e0_f64) * t1285 * t1811 * t12732 * t1287 + F::cast_from(0.65854491829355115987e0_f64) * t59550 * t13130 - F::cast_from(0.19756347548806534796e1_f64) * t1234 * t3759 * t16750 - F::cast_from(0.19756347548806534796e1_f64) * t5326 * t13121 - F::cast_from(0.19756347548806534796e1_f64) * t1234 * t13133 * t5245 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t5486 * t12621 + F::cast_from(0.65854491829355115987e0_f64) * t12690 * t1825 + F::cast_from(0.39512695097613069591e1_f64) * t3670 * t17821 * t3568 + F::cast_from(0.65854491829355115987e0_f64) * t1770 * t13108 + F::cast_from(0.19756347548806534796e1_f64) * t17861 * t3778 - F::cast_from(0.65854491829355115987e0_f64) * t3755 * t57200 * t1287 - F::cast_from(0.19756347548806534796e1_f64) * t3755 * t57498 * t1287;
    t59579
}
