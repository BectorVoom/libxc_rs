//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 668/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk668<F: Float>(t3990: F, t3991: F, t875: F, t3989: F, t1178: F, t371: F, t939: F, t1177: F, t1192: F, t2376: F, t830: F, t829: F) -> (F, F, F, F, F) {
    let t3993 = t3990 * t3991 * t875;
    let t3994 = t3989 * t3993;
    let t3997 = t371 * t1178 * t939;
    let t3998 = t1177 * t3997;
    let t4000 = t2376 * t1192;
    let t4001 = t830 * t4000;
    let t4002 = t829 * t4001;
    (t3993, t3994, t3997, t3998, t4002)
}
