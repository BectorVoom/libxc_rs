//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1675/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1675(t12657: f64, t3754: f64, t12722: f64, t3555: f64, t12640: f64, t3552: f64, t3766: f64, t5462: f64, t5477: f64, t12699: f64, t12709: f64, t12714: f64, t12719: f64, t12723: f64, t12727: f64, t12748: f64, t12753: f64, t12757: f64, t17955: f64, t3756: f64, t3770: f64, t3778: f64, t44639: f64, t5478: f64, t5480: f64) -> f64 {
    let t45697 = t12657 * t3754;
    let t45700 = t3555 * t12722;
    let t45707 = t12640 * t3754;
    let t45710 = t3552 * t3766;
    let t45715 = t3555 * t5462;
    let t45718 = t3555 * t5477;
    let t45723 = -0.79025390195226139183e1_f64 * t12723 * t12748 - 0.26341796731742046395e1_f64 * t5478 * t44639 * t5480 - 0.79025390195226139183e1_f64 * t45697 * t3756 - 0.15805078039045227836e2_f64 * t45700 * t3756 - 0.79025390195226139183e1_f64 * t12709 * t12727 - 0.79025390195226139183e1_f64 * t12709 * t12748 + 0.15805078039045227836e2_f64 * t45707 * t12719 + 0.79025390195226139183e1_f64 * t45710 * t3770 + 0.39512695097613069592e1_f64 * t12699 * t3778 - 0.15805078039045227836e2_f64 * t45715 * t12753 + 0.79025390195226139183e1_f64 * t45718 * t12757 + 0.15805078039045227836e2_f64 * t17955 * t12714;
    t45723
}
