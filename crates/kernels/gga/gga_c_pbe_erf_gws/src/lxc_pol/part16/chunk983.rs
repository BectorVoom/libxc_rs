//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 983/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk983<F: Float>(t14064: F, t854: F, t2308: F, t4039: F, t3065: F, t876: F, t2134: F, t1189: F, t2334: F, t2285: F, t4043: F, t2293: F, t2153: F, t899: F, t922: F) -> (F, F, F, F, F, F, F, F) {
    let t14065 = t854 * t14064;
    let t14067 = t4039 * t2308;
    let t14069 = t3065 * t876;
    let t14070 = t2134 * t14069;
    let t14072 = t1189 * t2334;
    let t14074 = t4043 * t2285;
    let t14076 = t4043 * t2293;
    let t14079 = t899 * t2153 * t922;
    (t14065, t14067, t14069, t14070, t14072, t14074, t14076, t14079)
}
