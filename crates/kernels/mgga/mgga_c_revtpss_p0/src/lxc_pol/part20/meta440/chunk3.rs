//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1674/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1674<F: Float>(t1209: F, t17845: F, t1214: F, t13043: F, t17852: F, t12627: F, t3754: F, t17847: F, t3588: F, t17854: F, t17887: F, t12717: F, t12723: F, t12727: F, t12753: F, t1287: F, t13143: F, t13149: F, t17846: F, t17853: F, t3755: F, t44321: F, t44585: F, t44599: F, t44610: F, t44618: F, t44753: F, t490: F) -> F {
    let t45654 = t1209 * t17845;
    let t45655 = t1214 * t13043;
    let t45659 = t1209 * t17852;
    let t45666 = t12627 * t3754;
    let t45675 = t17847 * t3588;
    let t45679 = t17854 * t3588;
    let t45683 = t1209 * t17887;
    let t45691 = -F::cast_from(0.15805078039045227836e2_f64) * t45654 * t45655 * t13149 + F::cast_from(0.15805078039045227836e2_f64) * t45659 * t45655 * t13143 - F::cast_from(0.39512695097613069592e1_f64) * t3755 * t44753 * t1287 - F::cast_from(0.15805078039045227836e2_f64) * t45666 * t44610 * t1287 - F::cast_from(0.79025390195226139183e1_f64) * t12723 * t12727 + F::cast_from(0.79025390195226139183e1_f64) * t12717 * t44618 * t1287 + F::cast_from(0.23707617058567841754e2_f64) * t17846 * t44585 * t45675 - F::cast_from(0.23707617058567841754e2_f64) * t17853 * t44585 * t45679 - F::cast_from(0.15805078039045227836e2_f64) * t45683 * t12753 - F::cast_from(0.26341796731742046395e1_f64) * t3755 * t44599 * t1287 + F::cast_from(0.65854491829355115987e0_f64) * t44321 * t490;
    t45691
}
