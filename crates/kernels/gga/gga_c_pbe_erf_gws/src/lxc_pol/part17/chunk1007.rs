//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1007/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1007<F: Float>(t2376: F, t4182: F, t829: F, t830: F, t1162: F, t875: F, t13796: F, t3989: F, t1105: F, t4052: F, t2409: F, t4164: F, t8734: F, t14423: F, t2171: F, t13859: F) -> (F, F, F, F, F, F, F, F) {
    let t14435 = t2376 * t4182;
    let t14437 = t829 * t830 * t14435;
    let t14442 = t1162 * t875;
    let t14443 = t13796 * t14442;
    let t14444 = t3989 * t14443;
    let t14446 = t4052 * t1105;
    let t14448 = t2409 * t2376 * t14446;
    let t14452 = t2409 * t8734 * t4164;
    let t14455 = t14423 * t2171;
    let t14456 = t13796 * t14455;
    let t14457 = t13859 * t14456;
    (t14437, t14443, t14444, t14446, t14448, t14452, t14456, t14457)
}
