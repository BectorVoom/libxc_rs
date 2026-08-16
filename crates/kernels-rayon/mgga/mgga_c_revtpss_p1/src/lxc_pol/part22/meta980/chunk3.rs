//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3305/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3305(t18615: f64, t251: f64, t231: f64, t2782: f64, t2783: f64, t10069: f64, t18738: f64, t18742: f64, t10073: f64, t18677: f64, t18681: f64, t2724: f64, t4504: f64, t62615: f64, t62619: f64, t62626: f64, t62630: f64, t62633: f64, t62635: f64, t62639: f64) -> (f64, f64) {
    let t62641 = t251 * t18615;
    let t62644 = t2782 * t2783 * t62641 * t231;
    let t62649 = t10069 * t18738;
    let t62651 = t10069 * t18742;
    let t62653 = t10073 * t18738;
    let t62655 = 0.10975748638225852664e-1_f64 * t62615 + 0.19514881078765566038e-1_f64 * t62619 + 0.92196288561097162379e1_f64 * t4504 * t18677 * t2724 + 0.21951497276451705328e-1_f64 * t62626 - 0.43902994552903410656e-1_f64 * t62630 + 0.13009920719177044025e-1_f64 * t62633 - 0.19514881078765566038e-1_f64 * t62635 + 0.10975748638225852664e-1_f64 * t62639 + 0.10975748638225852664e-1_f64 * t62644 + 0.79025390195226139182e1_f64 * t4504 * t18681 * t2724 - 0.73171657588172351096e-2_f64 * t62649 - 0.73171657588172351096e-2_f64 * t62651 + 0.65049603595885220126e-3_f64 * t62653;
    (t62641, t62655)
}
