//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1126/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1126<F: Float>(t14463: F, t3959: F, t2409: F, t8708: F, t1119: F, t4386: F) -> (F, F, F, F) {
    let t14464 = t3959 * t14463;
    let t14466 = t2409 * t8708;
    let t14467 = t3959 * t14466;
    let t14469 = t4386 * t1119;
    (t14464, t14466, t14467, t14469)
}
