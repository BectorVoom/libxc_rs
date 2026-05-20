//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3225/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3225<F: Float>(t17600: F, t3153: F, t12722: F, t5219: F, t12629: F, t12706: F, t12709: F, t12714: F, t12727: F, t12748: F, t12751: F, t12756: F, t13118: F, t16757: F, t17188: F, t17905: F, t17955: F, t17958: F, t21500: F, t21579: F, t3756: F, t3769: F, t3783: F, t45666: F, t45707: F, t5351: F, t5436: F, t5457: F, t5480: F, t57696: F) -> (F, F) {
    let t59699 = t17600 * t3153;
    let t59705 = t5219 * t12722;
    let t59724 = F::cast_from(0.39512695097613069591e1_f64) * t21500 * t12714 - F::cast_from(0.19756347548806534796e1_f64) * t21579 * t12706 + F::cast_from(0.19756347548806534796e1_f64) * t5436 * t13118 + F::cast_from(0.79025390195226139182e1_f64) * t17955 * t16757 + F::cast_from(0.19756347548806534796e1_f64) * t12756 * t59699 * t5480 + F::cast_from(0.79025390195226139182e1_f64) * t45707 * t17188 - F::cast_from(0.39512695097613069591e1_f64) * t59705 * t3756 - F::cast_from(0.19756347548806534796e1_f64) * t17958 * t12727 - F::cast_from(0.19756347548806534796e1_f64) * t17958 * t12748 - F::cast_from(0.39512695097613069591e1_f64) * t45666 * t5351 * t5457 * t12629 - F::cast_from(0.39512695097613069591e1_f64) * t12751 * t57696 * t3769 + F::cast_from(0.19756347548806534796e1_f64) * t12756 * t57696 * t3783 - F::cast_from(0.19756347548806534796e1_f64) * t12709 * t17905;
    (t59699, t59724)
}
