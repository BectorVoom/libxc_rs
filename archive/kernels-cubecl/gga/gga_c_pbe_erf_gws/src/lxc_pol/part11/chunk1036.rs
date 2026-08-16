//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1036/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1036<F: Float>(t1105: F, t1109: F, t3886: F, t8589: F, t829: F, t830: F, t376: F, t3772: F, t13173: F, t2366: F, t833: F, t13207: F, t4414: F) -> (F, F, F, F, F) {
    let t43357 = t1105 * t1109;
    let t43373 = t8589 * t3886;
    let t43375 = t829 * t830 * t43373;
    let t43451 = t376 * t3772;
    let t43466 = t13173 * t2366 * t833;
    let t43487 = t4414 * t13207;
    (t43357, t43375, t43451, t43466, t43487)
}
