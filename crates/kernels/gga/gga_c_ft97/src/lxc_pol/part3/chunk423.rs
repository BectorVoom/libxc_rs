//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 423/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk423<F: Float>(t103: F, t3170: F, t82: F, t376: F, t89: F, t973: F, t487: F, t979: F, t379: F, t1909: F, t1570: F, t363: F, t920: F, t100: F, t1780: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3172 = t82 * t3170 * t103;
    let t3177 = t89 * t376 * t973;
    let t3182 = t487 * t979;
    let t3183 = t3182 * t379;
    let t3184 = t1909 * t3183;
    let t3187 = t103 * t1570;
    let t3188 = t920 * t363;
    let t3189 = t3187 * t3188;
    let t3190 = t1909 * t3189;
    let t3193 = t1780 * t100;
    (t3172, t3177, t3183, t3184, t3187, t3188, t3189, t3190, t3193)
}
