//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3229/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3229<F: Float>(t3781: F, t5216: F, t45618: F, t460: F, t487: F, t43350: F, t44535: F, t45607: F, t13045: F, t1234: F, t12744: F, t1280: F, t1285: F, t1287: F, t12975: F, t12987: F, t13043: F, t13107: F, t13153: F, t17183: F, t1774: F, t17846: F, t17853: F, t17869: F, t1794: F, t3782: F, t3783: F, t3784: F, t45624: F, t5487: F, t56620: F, t56766: F, t57578: F, t58921: F, t59476: F, t59650: F) -> F {
    let t59854 = t5216 * t3781;
    let t59864 = t460 * t45618 * t487;
    let t59865 = t43350 * t44535;
    let t59871 = t460 * t45607 * t487;
    let t59872 = t43350 * t13045;
    let t59877 = -F::cast_from(0.19756347548806534796e1_f64) * t12975 * t5487 - F::cast_from(0.19756347548806534796e1_f64) * t3782 * t59476 * t3783 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t13107 * t1794 * t1287 - F::cast_from(0.19756347548806534796e1_f64) * t12744 * t17869 - F::cast_from(0.19756347548806534796e1_f64) * t17183 * t13153 + F::cast_from(0.11853808529283920877e2_f64) * t17846 * t59650 * t57578 - F::cast_from(0.11853808529283920877e2_f64) * t17853 * t59650 * t56766 - F::cast_from(0.19756347548806534796e1_f64) * t59854 * t3784 - F::cast_from(0.11853808529283920877e2_f64) * t12987 * t1280 * t56620 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t45624 * t1774 + F::cast_from(0.15805078039045227836e2_f64) * t59864 * t58921 * t59865 * t13043 - F::cast_from(0.23707617058567841754e2_f64) * t59871 * t58921 * t59872 * t13043;
    t59877
}
