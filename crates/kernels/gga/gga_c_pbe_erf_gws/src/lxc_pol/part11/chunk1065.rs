//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1065/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1065<F: Float>(t1109: F, t13290: F, t1161: F, t12130: F, t13105: F, t13171: F, t13173: F, t13205: F, t13217: F, t13220: F, t13662: F, t13688: F, t2376: F, t2408: F, t2409: F, t28394: F, t3052: F, t36041: F, t3920: F, t3921: F, t39466: F, t43466: F, t43487: F, t43872: F, t829: F, t830: F, t831: F, t833: F, t8589: F, t9241: F, t9820: F, t9849: F, t9890: F) -> (F, F) {
    let t49022 = t13290 * t1109;
    let t49058 = 11.0 / 96.0 * t36041 * t13662 - t28394 * t829 * t830 * t831 * t49022 / 16.0 + t12130 * t829 * t830 * t831 * t13171 / 24.0 + t13173 * t3920 * t833 / 32.0 + 3.0 / 8.0 * t3921 * t9820 - t13688 * t3052 / 6.0 - t9849 * t13217 / 32.0 - 7.0 / 72.0 * t43466 - 7.0 / 12.0 * t43487 + t2408 * t2409 * t8589 * t13205 / 4.0 + t9241 * t2409 * t2376 * t13220 * t1161 + t43872 * t13105 / 16.0 + t39466 * t13662 / 16.0 - t3921 * t9890 / 8.0;
    (t49022, t49058)
}
