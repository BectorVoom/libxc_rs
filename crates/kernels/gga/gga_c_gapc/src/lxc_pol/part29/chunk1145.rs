//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1145/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1145<F: Float>(t3327: F, t33655: F, t33685: F, t7073: F, t3751: F, t9635: F, t11954: F, t3392: F, t11957: F, t2387: F, t3297: F, t3761: F) -> (F, F, F, F, F) {
    let t34142 = t7073 * t33655 * t3327 * t33685;
    let t34144 = t3751 * t9635;
    let t34146 = t11954 * t3392;
    let t34148 = t11957 * t3392;
    let t34151 = t2387 * t3761 * t3297;
    (t34142, t34144, t34146, t34148, t34151)
}
