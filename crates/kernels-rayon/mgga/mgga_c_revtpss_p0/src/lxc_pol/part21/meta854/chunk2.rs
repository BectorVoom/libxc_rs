//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3225/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3225(t17600: f64, t3153: f64, t12722: f64, t5219: f64, t12629: f64, t12706: f64, t12709: f64, t12714: f64, t12727: f64, t12748: f64, t12751: f64, t12756: f64, t13118: f64, t16757: f64, t17188: f64, t17905: f64, t17955: f64, t17958: f64, t21500: f64, t21579: f64, t3756: f64, t3769: f64, t3783: f64, t45666: f64, t45707: f64, t5351: f64, t5436: f64, t5457: f64, t5480: f64, t57696: f64) -> (f64, f64) {
    let t59699 = t17600 * t3153;
    let t59705 = t5219 * t12722;
    let t59724 = 0.39512695097613069591e1_f64 * t21500 * t12714 - 0.19756347548806534796e1_f64 * t21579 * t12706 + 0.19756347548806534796e1_f64 * t5436 * t13118 + 0.79025390195226139182e1_f64 * t17955 * t16757 + 0.19756347548806534796e1_f64 * t12756 * t59699 * t5480 + 0.79025390195226139182e1_f64 * t45707 * t17188 - 0.39512695097613069591e1_f64 * t59705 * t3756 - 0.19756347548806534796e1_f64 * t17958 * t12727 - 0.19756347548806534796e1_f64 * t17958 * t12748 - 0.39512695097613069591e1_f64 * t45666 * t5351 * t5457 * t12629 - 0.39512695097613069591e1_f64 * t12751 * t57696 * t3769 + 0.19756347548806534796e1_f64 * t12756 * t57696 * t3783 - 0.19756347548806534796e1_f64 * t12709 * t17905;
    (t59699, t59724)
}
