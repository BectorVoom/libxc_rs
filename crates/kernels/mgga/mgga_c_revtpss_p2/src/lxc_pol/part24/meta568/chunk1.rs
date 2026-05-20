//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1741/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1741<F: Float>(t89947: F, t89959: F, t6573: F, t6628: F, t1774: F, t22688: F, t1042: F, t1261: F, t17202: F, t17569: F, t24612: F, t24773: F, t3711: F, t5268: F, t5293: F, t5819: F, t6587: F, t69668: F, t69700: F, t82338: F, t82351: F, t82434: F, t82441: F, t88732: F) -> (F, F, F, F) {
    let t89960 = t89947 + t89959;
    let t89978 = t6573 * t6628;
    let t90001 = t22688 * t1774;
    let t90012 = -F::cast_from(0.45732285992607719436e-2_f64) * t5293 * t24773 - F::cast_from(0.51448821741683684366e-2_f64) * t1261 * t1042 * t17202 * t88732 + F::cast_from(0.34299214494455789577e-2_f64) * t17569 * t24612 + F::cast_from(0.17149607247227894789e-2_f64) * t3711 * t1042 * t5268 * t5819 * t6587 + F::cast_from(0.34299214494455789577e-2_f64) * t3711 * t1042 * t17202 * t90001 - F::cast_from(0.91464571985215438872e-2_f64) * t82338 + F::cast_from(0.22866142996303859718e-2_f64) * t82351 - F::cast_from(0.28582678745379824648e-3_f64) * t69668 - F::cast_from(0.57165357490759649296e-3_f64) * t69700 + F::cast_from(0.91464571985215438872e-2_f64) * t82434 + F::cast_from(0.57927562257303111285e-1_f64) * t82441;
    (t89960, t89978, t90001, t90012)
}
