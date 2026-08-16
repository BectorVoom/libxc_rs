//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3315/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3315(t14602: f64, t14961: f64, t1558: f64, t2482: f64, t4469: f64, t14520: f64, t14568: f64, t14524: f64, t51297: f64, t2801: f64, t4526: f64, t14546: f64, t14547: f64, t18699: f64, t2724: f64, t4366: f64, t4494: f64, t4504: f64, t51519: f64, t51521: f64, t51523: f64, t51527: f64, t51531: f64, t62209: f64) -> (f64, f64) {
    let t62866 = t2482 * t14961 * t1558 * t14602;
    let t62868 = t4469 * t1558;
    let t62872 = t14568 * t14520;
    let t62874 = t51297 * t14524;
    let t62881 = t2482 * t4526 * t1558 * t2801;
    let t62887 = 0.39512695097613069591e1_f64 * t4504 * t18699 * t2724 + 0.10975748638225852664e-1_f64 * t51519 + 0.2601984143835408805e-2_f64 * t51521 - 0.19514881078765566038e-1_f64 * t51523 + 0.10975748638225852664e-1_f64 * t51527 + 0.78059524315062264149e-1_f64 * t62866 + 0.52683593463484092788e1_f64 * t4504 * t62868 * t4366 - 0.39029762157531132074e-1_f64 * t62872 - 0.23131639038696784277e-2_f64 * t62874 - 0.39512695097613069591e1_f64 * t14546 * t18699 * t14547 - 0.39029762157531132074e-1_f64 * t62881 + 0.26341796731742046394e1_f64 * t4504 * t4494 * t62209 - 0.43902994552903410656e-1_f64 * t51531;
    (t62868, t62887)
}
