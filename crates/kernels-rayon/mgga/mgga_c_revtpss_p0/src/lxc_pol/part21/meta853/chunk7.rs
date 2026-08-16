//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3219/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3219(t17482: f64, t3153: f64, t1284: f64, t17331: f64, t12646: f64, t12699: f64, t12702: f64, t12734: f64, t12737: f64, t1288: f64, t12966: f64, t13133: f64, t13156: f64, t17307: f64, t17811: f64, t17815: f64, t21483: f64, t3670: f64, t3751: f64, t45859: f64, t5230: f64, t5436: f64, t5463: f64, t5465: f64, t5470: f64, t5478: f64, t5480: f64, t5486: f64, t57373: f64, t59241: f64) -> (f64, f64) {
    let t59514 = t17482 * t3153;
    let t59537 = t17331 * t1284;
    let t59544 = 0.39512695097613069591e1_f64 * t5463 * t57373 * t5465 + 0.79025390195226139182e1_f64 * t45859 * t59514 * t21483 - 0.19756347548806534796e1_f64 * t5478 * t57373 * t5480 + 0.39512695097613069591e1_f64 * t3670 * t5486 * t12646 + 0.79025390195226139182e1_f64 * t12966 * t17811 + 0.65854491829355115987e0_f64 * t5436 * t12734 + 0.39512695097613069591e1_f64 * t17307 * t13156 + 0.39512695097613069591e1_f64 * t3670 * t13133 * t5230 + 0.39512695097613069591e1_f64 * t59241 * t3751 + 0.39512695097613069591e1_f64 * t17307 * t12737 + 0.19756347548806534796e1_f64 * t59537 * t1288 + 0.39512695097613069591e1_f64 * t12702 * t17815 + 0.19756347548806534796e1_f64 * t12699 * t5470;
    (t59514, t59544)
}
