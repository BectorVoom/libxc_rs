//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2921/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2921<F: Float>(t341: F, t52954: F, t52975: F, t4930: F, t989: F, t1079: F, t1097: F, t11183: F, t11184: F, t11187: F, t11210: F, t11214: F, t11902: F, t16243: F, t16249: F, t16255: F, t16292: F, t16312: F, t16313: F, t16314: F, t16322: F, t16362: F, t16591: F, t16597: F, t1680: F, t3043: F, t3047: F, t3076: F, t3261: F, t3264: F, t3326: F, t386: F, t43656: F, t4743: F, t4747: F, t4758: F, t4773: F, t4932: F, t4947: F, t52927: F, t995: F, t999: F) -> (F, F) {
    let t52977 = (t52954 + t52975) * t341;
    let t52994 = t989 * t4930;
    let t53011 = F::cast_from(0.79025390195226139182e1_f64) * t3264 * t16255 - F::cast_from(0.79025390195226139182e1_f64) * t52927 * t16314 - F::cast_from(0.19756347548806534796e1_f64) * t16362 * t3326 - F::cast_from(0.39512695097613069591e1_f64) * t16312 * t16313 * t11183 + F::cast_from(0.65854491829355115987e0_f64) * t52977 * t386 + F::cast_from(0.19756347548806534796e1_f64) * t995 * t1079 * t16591 * t999 - F::cast_from(0.19756347548806534796e1_f64) * t11214 * t4773 - F::cast_from(0.39512695097613069591e1_f64) * t3047 * t16249 + F::cast_from(0.39512695097613069591e1_f64) * t3047 * t16243 - F::cast_from(0.11853808529283920877e2_f64) * t3264 * t16322 + F::cast_from(0.19756347548806534796e1_f64) * t3043 * t4932 - F::cast_from(0.39512695097613069591e1_f64) * t52994 * t1097 + F::cast_from(0.65854491829355115987e0_f64) * t11902 * t1680 + F::cast_from(0.39512695097613069591e1_f64) * t11210 * t4947 + F::cast_from(0.19756347548806534796e1_f64) * t4747 * t11184 + F::cast_from(0.19756347548806534796e1_f64) * t4743 * t3261 + F::cast_from(0.79025390195226139182e1_f64) * t43656 * t4758 + F::cast_from(0.79025390195226139182e1_f64) * t11187 * t16292 - F::cast_from(0.19756347548806534796e1_f64) * t16597 * t3076;
    (t52977, t53011)
}
