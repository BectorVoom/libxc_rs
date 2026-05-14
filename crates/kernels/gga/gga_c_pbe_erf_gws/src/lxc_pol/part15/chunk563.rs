//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 563/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk563<F: Float>(t2522: F, t41: F, t700: F, t992: F, t1072: F, t168: F, t703: F, t1632: F, t1653: F, t1069: F, t735: F, t92: F, t950: F, t34: F, t726: F, t93: F, t954: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2523 = t41 * t2522;
    let t2526 = t992 * t700;
    let t2531 = t168 * t703 * t1072;
    let t2534 = 8.0 / 135.0 * t1632;
    let t2535 = 8.0 / 135.0 * t1653;
    let t2536 = t1069 * t735;
    let t2538 = t92 * t950;
    let t2541 = t726 * t34;
    let t2544 = t93 * t954;
    (t2523, t2526, t2531, t2534, t2535, t2536, t2538, t2541, t2544)
}
