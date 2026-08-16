//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3313/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3313(t1568: f64, t4423: f64, t2783: f64, t6041: f64, t786: f64, t2801: f64, t10943: f64, t14546: f64, t18525: f64, t18616: f64, t18681: f64, t2646: f64, t2754: f64, t2815: f64, t40267: f64, t40271: f64, t40273: f64, t40294: f64, t4366: f64, t4494: f64, t4504: f64, t4514: f64, t51505: f64, t51507: f64, t62760: f64, t820: f64, t837: f64) -> (f64, f64) {
    let t62803 = t1568 * t4423;
    let t62808 = t786 * t2783 * t6041;
    let t62809 = t62808 * t2801;
    let t62825 = -0.14634331517634470219e-1_f64 * t40267 - 0.52039682876708176102e-2_f64 * t40271 + 0.65049603595885220126e-3_f64 * t40273 - 0.15805078039045227836e2_f64 * t14546 * t4494 * t18525 * t4423 - 0.13170898365871023197e1_f64 * t820 * t2815 * t18616 - 0.26341796731742046394e1_f64 * t4514 * t62803 * t837 - t40294 - 0.19514881078765566038e-1_f64 * t62809 + 0.26341796731742046394e1_f64 * t4504 * t18681 * t10943 - 0.13170898365871023197e1_f64 * t4514 * t18681 * t2754 + 0.79025390195226139182e1_f64 * t4504 * t62760 * t4366 - 0.10975748638225852664e-1_f64 * t51505 - 0.13170898365871023197e1_f64 * t4514 * t18681 * t2646 + 0.58537326070537880875e-1_f64 * t51507;
    (t62803, t62825)
}
