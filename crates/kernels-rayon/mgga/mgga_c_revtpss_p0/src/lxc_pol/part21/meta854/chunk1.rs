//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3224/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3224(t12621: f64, t12699: f64, t12748: f64, t12757: f64, t1280: f64, t13161: f64, t17192: f64, t17849: f64, t17949: f64, t3670: f64, t3755: f64, t3756: f64, t45738: f64, t5351: f64, t5457: f64, t5458: f64, t5474: f64, t56543: f64, t57275: f64, t57325: f64, t57465: f64, t58785: f64, t59650: f64, t59657: f64, t59671: f64, t59674: f64, t59681: f64, t59686: f64) -> f64 {
    let t59689 = -0.19756347548806534796e1_f64 * t45738 * t59650 * t58785 + 0.19756347548806534796e1_f64 * t17949 * t59650 * t57325 - 0.39512695097613069591e1_f64 * t59657 * t3756 + 0.13170898365871023197e1_f64 * t3670 * t1280 * t56543 + 0.19756347548806534796e1_f64 * t12699 * t5474 - 0.39512695097613069591e1_f64 * t57465 * t13161 - 0.19756347548806534796e1_f64 * t3755 * t57275 * t5458 + 0.11853808529283920877e2_f64 * t59671 * t17849 + 0.19756347548806534796e1_f64 * t59674 * t12757 - 0.65854491829355115987e0_f64 * t3755 * t5351 * t5457 * t12621 + 0.19756347548806534796e1_f64 * t59681 * t12757 - 0.19756347548806534796e1_f64 * t17192 * t12748 - 0.39512695097613069591e1_f64 * t59686 * t3756;
    t59689
}
