//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1674/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1674(t1209: f64, t17845: f64, t1214: f64, t13043: f64, t17852: f64, t12627: f64, t3754: f64, t17847: f64, t3588: f64, t17854: f64, t17887: f64, t12717: f64, t12723: f64, t12727: f64, t12753: f64, t1287: f64, t13143: f64, t13149: f64, t17846: f64, t17853: f64, t3755: f64, t44321: f64, t44585: f64, t44599: f64, t44610: f64, t44618: f64, t44753: f64, t490: f64) -> f64 {
    let t45654 = t1209 * t17845;
    let t45655 = t1214 * t13043;
    let t45659 = t1209 * t17852;
    let t45666 = t12627 * t3754;
    let t45675 = t17847 * t3588;
    let t45679 = t17854 * t3588;
    let t45683 = t1209 * t17887;
    let t45691 = -0.15805078039045227836e2_f64 * t45654 * t45655 * t13149 + 0.15805078039045227836e2_f64 * t45659 * t45655 * t13143 - 0.39512695097613069592e1_f64 * t3755 * t44753 * t1287 - 0.15805078039045227836e2_f64 * t45666 * t44610 * t1287 - 0.79025390195226139183e1_f64 * t12723 * t12727 + 0.79025390195226139183e1_f64 * t12717 * t44618 * t1287 + 0.23707617058567841754e2_f64 * t17846 * t44585 * t45675 - 0.23707617058567841754e2_f64 * t17853 * t44585 * t45679 - 0.15805078039045227836e2_f64 * t45683 * t12753 - 0.26341796731742046395e1_f64 * t3755 * t44599 * t1287 + 0.65854491829355115987e0_f64 * t44321 * t490;
    t45691
}
