//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 963/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk963(t12032: f64, t2902: f64, t14295: f64, t4342: f64, t12148: f64, t2798: f64, t44671: f64, t44674: f64, t44676: f64, t44678: f64, t44684: f64, t44687: f64, t44689: f64, t44692: f64, t44694: f64, t44705: f64, t45134: f64, t45148: f64, t45151: f64, t49820: f64, t49965: f64, t49968: f64) -> (f64, f64, f64, f64) {
    let t49970 = 2.0_f64 * t12032 * t2902;
    let t49972 = 4.0_f64 * t4342 * t14295;
    let t49974 = 2.0_f64 * t2798 * t12148;
    let t49975 = -t44671 - t44674 - t49820 + t44676 - t44678 - t44684 + t44687 - t44689 + t44692 - t44694 - t44705 + t49965 + t49968 - t49970 + t45134 + t45148 - t45151 + t49972 - t49974;
    (t49970, t49972, t49974, t49975)
}
