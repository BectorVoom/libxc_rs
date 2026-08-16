//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 968/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk968<F: Float>(t11257: F, t11258: F, t3650: F, t4865: F, t11235: F, t4868: F, t2922: F, t3646: F, t3643: F, t8492: F, t3694: F, t5: F) -> (F, F, F, F, F, F, F) {
    let t11259 = t11257 * t11258;
    let t11261 = t3650 * t4865;
    let t11262 = t11235 * t4868;
    let t11263 = t11261 * t11262;
    let t11265 = t2922 * t3646;
    let t11267 = t3643 * t8492;
    let t11268 = t11267 * t3646;
    let t11270 = t5 * t3694;
    (t11259, t11261, t11262, t11263, t11265, t11268, t11270)
}
