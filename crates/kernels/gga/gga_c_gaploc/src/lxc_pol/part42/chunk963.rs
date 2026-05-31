//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 963/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk963<F: Float>(t12032: F, t2902: F, t14295: F, t4342: F, t12148: F, t2798: F, t44671: F, t44674: F, t44676: F, t44678: F, t44684: F, t44687: F, t44689: F, t44692: F, t44694: F, t44705: F, t45134: F, t45148: F, t45151: F, t49820: F, t49965: F, t49968: F) -> (F, F, F, F) {
    let t49970 = F::cast_from(2.0_f64) * t12032 * t2902;
    let t49972 = F::cast_from(4.0_f64) * t4342 * t14295;
    let t49974 = F::cast_from(2.0_f64) * t2798 * t12148;
    let t49975 = -t44671 - t44674 - t49820 + t44676 - t44678 - t44684 + t44687 - t44689 + t44692 - t44694 - t44705 + t49965 + t49968 - t49970 + t45134 + t45148 - t45151 + t49972 - t49974;
    (t49970, t49972, t49974, t49975)
}
