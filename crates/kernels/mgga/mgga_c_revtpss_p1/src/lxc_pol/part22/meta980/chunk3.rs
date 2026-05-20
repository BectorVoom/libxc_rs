//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3305/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3305<F: Float>(t18615: F, t251: F, t231: F, t2782: F, t2783: F, t10069: F, t18738: F, t18742: F, t10073: F, t18677: F, t18681: F, t2724: F, t4504: F, t62615: F, t62619: F, t62626: F, t62630: F, t62633: F, t62635: F, t62639: F) -> (F, F) {
    let t62641 = t251 * t18615;
    let t62644 = t2782 * t2783 * t62641 * t231;
    let t62649 = t10069 * t18738;
    let t62651 = t10069 * t18742;
    let t62653 = t10073 * t18738;
    let t62655 = F::cast_from(0.10975748638225852664e-1_f64) * t62615 + F::cast_from(0.19514881078765566038e-1_f64) * t62619 + F::cast_from(0.92196288561097162379e1_f64) * t4504 * t18677 * t2724 + F::cast_from(0.21951497276451705328e-1_f64) * t62626 - F::cast_from(0.43902994552903410656e-1_f64) * t62630 + F::cast_from(0.13009920719177044025e-1_f64) * t62633 - F::cast_from(0.19514881078765566038e-1_f64) * t62635 + F::cast_from(0.10975748638225852664e-1_f64) * t62639 + F::cast_from(0.10975748638225852664e-1_f64) * t62644 + F::cast_from(0.79025390195226139182e1_f64) * t4504 * t18681 * t2724 - F::cast_from(0.73171657588172351096e-2_f64) * t62649 - F::cast_from(0.73171657588172351096e-2_f64) * t62651 + F::cast_from(0.65049603595885220126e-3_f64) * t62653;
    (t62641, t62655)
}
