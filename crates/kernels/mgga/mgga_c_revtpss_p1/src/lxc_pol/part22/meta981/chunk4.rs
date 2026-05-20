//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3315/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3315<F: Float>(t14602: F, t14961: F, t1558: F, t2482: F, t4469: F, t14520: F, t14568: F, t14524: F, t51297: F, t2801: F, t4526: F, t14546: F, t14547: F, t18699: F, t2724: F, t4366: F, t4494: F, t4504: F, t51519: F, t51521: F, t51523: F, t51527: F, t51531: F, t62209: F) -> (F, F) {
    let t62866 = t2482 * t14961 * t1558 * t14602;
    let t62868 = t4469 * t1558;
    let t62872 = t14568 * t14520;
    let t62874 = t51297 * t14524;
    let t62881 = t2482 * t4526 * t1558 * t2801;
    let t62887 = F::cast_from(0.39512695097613069591e1_f64) * t4504 * t18699 * t2724 + F::cast_from(0.10975748638225852664e-1_f64) * t51519 + F::cast_from(0.2601984143835408805e-2_f64) * t51521 - F::cast_from(0.19514881078765566038e-1_f64) * t51523 + F::cast_from(0.10975748638225852664e-1_f64) * t51527 + F::cast_from(0.78059524315062264149e-1_f64) * t62866 + F::cast_from(0.52683593463484092788e1_f64) * t4504 * t62868 * t4366 - F::cast_from(0.39029762157531132074e-1_f64) * t62872 - F::cast_from(0.23131639038696784277e-2_f64) * t62874 - F::cast_from(0.39512695097613069591e1_f64) * t14546 * t18699 * t14547 - F::cast_from(0.39029762157531132074e-1_f64) * t62881 + F::cast_from(0.26341796731742046394e1_f64) * t4504 * t4494 * t62209 - F::cast_from(0.43902994552903410656e-1_f64) * t51531;
    (t62868, t62887)
}
