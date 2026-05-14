//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 923/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk923<F: Float>(t13217: F, t8662: F, t13220: F, t831: F, t19693: F, t830: F, t3083: F, t9899: F, t2503: F, t9955: F, t1105: F, t1109: F, t3886: F, t8589: F, t829: F, t376: F, t3772: F) -> (F, F, F, F, F, F, F) {
    let t43304 = t8662 * t13217;
    let t43321 = t831 * t13220;
    let t43323 = t19693 * t830 * t43321;
    let t43328 = t3083 * t9899;
    let t43344 = t9955 * t2503;
    let t43357 = t1105 * t1109;
    let t43373 = t8589 * t3886;
    let t43375 = t829 * t830 * t43373;
    let t43451 = t376 * t3772;
    (t43304, t43323, t43328, t43344, t43357, t43375, t43451)
}
