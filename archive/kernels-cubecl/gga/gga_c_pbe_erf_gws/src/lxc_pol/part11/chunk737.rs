//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 737/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk737<F: Float>(t2206: F, t3799: F, t11583: F, t337: F, t6560: F, t2323: F, t3875: F, t3128: F, t8833: F, t3703: F, t6: F, t2142: F, t3805: F) -> (F, F, F, F, F, F, F) {
    let t11922 = t2206 * t3799;
    let t11924 = t337 * t11583;
    let t11925 = t6560 * t11924;
    let t11944 = t2323 * t3875;
    let t11946 = t3128 * t8833;
    let t11964 = t6 * t3703;
    let t11975 = t3805 * t2142;
    (t11922, t11924, t11925, t11944, t11946, t11964, t11975)
}
