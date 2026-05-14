//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 986/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk986<F: Float>(t11948: F, t29350: F, t10039: F, t3438: F, t772: F, t11479: F, t2767: F, t7294: F, t11748: F, t2594: F, t2600: F, t11804: F, t11814: F, t2599: F, t11325: F, t3402: F) -> (F, F, F, F, F, F, F) {
    let t33187 = t11948 * t29350;
    let t33190 = t3438 * t772 * t10039;
    let t33193 = t7294 * t11479 * t2767;
    let t33195 = t11748 * t2594;
    let t33197 = t11748 * t2600;
    let t33200 = t11814 * t11804 * t2599;
    let t33202 = t3402 * t11325;
    (t33187, t33190, t33193, t33195, t33197, t33200, t33202)
}
