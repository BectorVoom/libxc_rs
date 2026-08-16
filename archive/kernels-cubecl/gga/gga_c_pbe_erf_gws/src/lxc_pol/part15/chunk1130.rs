//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1130/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1130<F: Float>(t14423: F, t2171: F, t13796: F, t13859: F, t2409: F, t4007: F, t8589: F, t9721: F, t3959: F, t8708: F, t1119: F, t4386: F) -> (F, F, F, F, F, F, F, F) {
    let t14455 = t14423 * t2171;
    let t14456 = t13796 * t14455;
    let t14457 = t13859 * t14456;
    let t14460 = t2409 * t8589 * t4007;
    let t14463 = t2409 * t9721;
    let t14464 = t3959 * t14463;
    let t14466 = t2409 * t8708;
    let t14467 = t3959 * t14466;
    let t14469 = t4386 * t1119;
    (t14456, t14457, t14460, t14463, t14464, t14466, t14467, t14469)
}
