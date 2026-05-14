//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 859/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk859<F: Float>(t10617: F, t3513: F, t7011: F, t4913: F, t2607: F, t2722: F, t1621: F, t1620: F, t2603: F, t2612: F, t3492: F, t586: F, t645: F, t2654: F, t5390: F, t3603: F, t735: F) -> (F, F, F, F, F, F, F, F) {
    let t10618 = 8.0 / 45.0 * t10617;
    let t10620 = 8.0 / 15.0 * t7011 * t3513;
    let t10622 = 8.0 / 15.0 * t4913 * t3513;
    let t10623 = t2607 * t2722;
    let t10624 = t1621 * t10623;
    let t10626 = 8.0 / 15.0 * t1620 * t10624;
    let t10628 = 8.0 / 15.0 * t2612 * t2603;
    let t10629 = t3492 * t586;
    let t10631 = 8.0 / 45.0 * t10629 * t645;
    let t10633 = 0.2e-20 * t2654 * t5390;
    let t10634 = t3603 * t735;
    (t10618, t10620, t10622, t10626, t10628, t10631, t10633, t10634)
}
