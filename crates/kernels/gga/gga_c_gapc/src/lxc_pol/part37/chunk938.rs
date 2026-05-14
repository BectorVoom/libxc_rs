//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 938/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk938<F: Float>(t11756: F, t11762: F, t11773: F, t11776: F, t12193: F, t12194: F, t12195: F, t12196: F, t12197: F, t12198: F, t12199: F, t12200: F, t12203: F, t12204: F, t12205: F, t12208: F, t12209: F, t12210: F, t12211: F) -> (F,) {
    let t12633 = t12193 + t12194 - t12195 + t12196 + t12197 - t12198 - t12199 + t12200 - 0.25297741735382421301e-7 * t11756 + 0.12228868272569444445e-4 * t11762 - t12203 - t12204 + t12205 + 0.12650553385416666667e-5 * t11773 + 0.12650553385416666667e-5 * t11776 + t12208 + t12209 + t12210 + t12211;
    (t12633,)
}
