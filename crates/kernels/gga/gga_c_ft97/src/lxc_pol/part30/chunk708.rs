//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 708/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk708<F: Float>(t193: F, t33840: F, t6308: F, t1486: F, t681: F, t7646: F, t2: F, t7584: F) -> (F, F, F, F) {
    let t33842 = t6308 * t193 * t33840;
    let t33845 = t1486 * t681 * t7646;
    let t33846 = t33845 / 9.0;
    let t33847 = t2 * t7584;
    (t33842, t33845, t33846, t33847)
}
