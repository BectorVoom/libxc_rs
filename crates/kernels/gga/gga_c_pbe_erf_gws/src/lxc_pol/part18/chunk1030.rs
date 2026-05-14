//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1030/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1030<F: Float>(t14622: F, t2409: F, t3067: F, t1192: F, t2494: F, t2376: F, t13780: F, t3212: F, t3990: F, t13859: F, t1176: F, t367: F, t6365: F) -> (F, F, F, F, F, F) {
    let t14624 = t2409 * t3067 * t14622;
    let t14627 = t1192 * t2494;
    let t14629 = t2409 * t2376 * t14627;
    let t14633 = t3990 * t13780 * t3212;
    let t14634 = t13859 * t14633;
    let t14637 = t1176 * t367 * t6365;
    (t14624, t14627, t14629, t14633, t14634, t14637)
}
