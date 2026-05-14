//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1034/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1034<F: Float>(t2164: F, t6442: F, t20182: F, t20761: F, t20769: F, t20781: F, t20785: F, t20786: F, t20791: F, t2272: F, t2338: F, t2343: F, t2345: F, t3235: F, t3247: F, t6282: F, t6360: F, t6579: F, t6580: F, t875: F) -> (F, F) {
    let t20792 = t2164 * t6442;
    let t20793 = 7.0 / 72.0 * t20792;
    let t20794 = t20761 + 5.0 / 64.0 * t6579 * t6580 * t2338 + 5.0 / 64.0 * t6579 * t6580 * t2272 - t20769 + 9.0 / 256.0 * t3247 * t3235 * t6282 * t6360 + t2343 * t2345 * t20182 * t875 / 96.0 + t20781 - t20785 - 7.0 / 48.0 * t20786 + t20791 + t20793;
    (t20793, t20794)
}
