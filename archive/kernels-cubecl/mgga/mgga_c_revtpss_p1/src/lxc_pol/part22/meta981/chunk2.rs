//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3313/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3313<F: Float>(t1568: F, t4423: F, t2783: F, t6041: F, t786: F, t2801: F, t10943: F, t14546: F, t18525: F, t18616: F, t18681: F, t2646: F, t2754: F, t2815: F, t40267: F, t40271: F, t40273: F, t40294: F, t4366: F, t4494: F, t4504: F, t4514: F, t51505: F, t51507: F, t62760: F, t820: F, t837: F) -> (F, F) {
    let t62803 = t1568 * t4423;
    let t62808 = t786 * t2783 * t6041;
    let t62809 = t62808 * t2801;
    let t62825 = -F::cast_from(0.14634331517634470219e-1_f64) * t40267 - F::cast_from(0.52039682876708176102e-2_f64) * t40271 + F::cast_from(0.65049603595885220126e-3_f64) * t40273 - F::cast_from(0.15805078039045227836e2_f64) * t14546 * t4494 * t18525 * t4423 - F::cast_from(0.13170898365871023197e1_f64) * t820 * t2815 * t18616 - F::cast_from(0.26341796731742046394e1_f64) * t4514 * t62803 * t837 - t40294 - F::cast_from(0.19514881078765566038e-1_f64) * t62809 + F::cast_from(0.26341796731742046394e1_f64) * t4504 * t18681 * t10943 - F::cast_from(0.13170898365871023197e1_f64) * t4514 * t18681 * t2754 + F::cast_from(0.79025390195226139182e1_f64) * t4504 * t62760 * t4366 - F::cast_from(0.10975748638225852664e-1_f64) * t51505 - F::cast_from(0.13170898365871023197e1_f64) * t4514 * t18681 * t2646 + F::cast_from(0.58537326070537880875e-1_f64) * t51507;
    (t62803, t62825)
}
