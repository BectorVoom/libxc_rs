//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3229/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3229(t3781: f64, t5216: f64, t45618: f64, t460: f64, t487: f64, t43350: f64, t44535: f64, t45607: f64, t13045: f64, t1234: f64, t12744: f64, t1280: f64, t1285: f64, t1287: f64, t12975: f64, t12987: f64, t13043: f64, t13107: f64, t13153: f64, t17183: f64, t1774: f64, t17846: f64, t17853: f64, t17869: f64, t1794: f64, t3782: f64, t3783: f64, t3784: f64, t45624: f64, t5487: f64, t56620: f64, t56766: f64, t57578: f64, t58921: f64, t59476: f64, t59650: f64) -> f64 {
    let t59854 = t5216 * t3781;
    let t59864 = t460 * t45618 * t487;
    let t59865 = t43350 * t44535;
    let t59871 = t460 * t45607 * t487;
    let t59872 = t43350 * t13045;
    let t59877 = -0.19756347548806534796e1_f64 * t12975 * t5487 - 0.19756347548806534796e1_f64 * t3782 * t59476 * t3783 + 0.65854491829355115987e0_f64 * t1285 * t13107 * t1794 * t1287 - 0.19756347548806534796e1_f64 * t12744 * t17869 - 0.19756347548806534796e1_f64 * t17183 * t13153 + 0.11853808529283920877e2_f64 * t17846 * t59650 * t57578 - 0.11853808529283920877e2_f64 * t17853 * t59650 * t56766 - 0.19756347548806534796e1_f64 * t59854 * t3784 - 0.11853808529283920877e2_f64 * t12987 * t1280 * t56620 - 0.65854491829355115987e0_f64 * t1234 * t45624 * t1774 + 0.15805078039045227836e2_f64 * t59864 * t58921 * t59865 * t13043 - 0.23707617058567841754e2_f64 * t59871 * t58921 * t59872 * t13043;
    t59877
}
