//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 941/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk941<F: Float>(t136595: F, t27519: F, t3771: F, t24260: F, t3766: F, t9: F, t420: F, t6044: F, t213: F, t37481: F, t7464: F, t36835: F, t5567: F) -> (F, F, F, F, F) {
    let t141089 = t3771 * t27519 * t136595;
    let t141096 = t3766 * t24260 * t9;
    let t141097 = t6044 * t420;
    let t141107 = t37481 * t213 * t7464;
    let t141111 = t36835 * t5567;
    (t141089, t141096, t141097, t141107, t141111)
}
