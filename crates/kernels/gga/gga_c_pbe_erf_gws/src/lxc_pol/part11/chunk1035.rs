//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1035/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1035<F: Float>(t13086: F, t831: F, t2370: F, t830: F, t13217: F, t8662: F, t13220: F, t19693: F, t3083: F, t9899: F, t2503: F, t9955: F) -> (F, F, F, F, F) {
    let t43288 = t831 * t13086;
    let t43290 = t2370 * t830 * t43288;
    let t43304 = t8662 * t13217;
    let t43321 = t831 * t13220;
    let t43323 = t19693 * t830 * t43321;
    let t43328 = t3083 * t9899;
    let t43344 = t9955 * t2503;
    (t43290, t43304, t43323, t43328, t43344)
}
