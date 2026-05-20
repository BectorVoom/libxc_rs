//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1675/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1675<F: Float>(t12657: F, t3754: F, t12722: F, t3555: F, t12640: F, t3552: F, t3766: F, t5462: F, t5477: F, t12699: F, t12709: F, t12714: F, t12719: F, t12723: F, t12727: F, t12748: F, t12753: F, t12757: F, t17955: F, t3756: F, t3770: F, t3778: F, t44639: F, t5478: F, t5480: F) -> F {
    let t45697 = t12657 * t3754;
    let t45700 = t3555 * t12722;
    let t45707 = t12640 * t3754;
    let t45710 = t3552 * t3766;
    let t45715 = t3555 * t5462;
    let t45718 = t3555 * t5477;
    let t45723 = -F::cast_from(0.79025390195226139183e1_f64) * t12723 * t12748 - F::cast_from(0.26341796731742046395e1_f64) * t5478 * t44639 * t5480 - F::cast_from(0.79025390195226139183e1_f64) * t45697 * t3756 - F::cast_from(0.15805078039045227836e2_f64) * t45700 * t3756 - F::cast_from(0.79025390195226139183e1_f64) * t12709 * t12727 - F::cast_from(0.79025390195226139183e1_f64) * t12709 * t12748 + F::cast_from(0.15805078039045227836e2_f64) * t45707 * t12719 + F::cast_from(0.79025390195226139183e1_f64) * t45710 * t3770 + F::cast_from(0.39512695097613069592e1_f64) * t12699 * t3778 - F::cast_from(0.15805078039045227836e2_f64) * t45715 * t12753 + F::cast_from(0.79025390195226139183e1_f64) * t45718 * t12757 + F::cast_from(0.15805078039045227836e2_f64) * t17955 * t12714;
    t45723
}
