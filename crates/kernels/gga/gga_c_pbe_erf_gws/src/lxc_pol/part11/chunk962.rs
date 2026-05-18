//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 962/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk962<F: Float>(t2970: F, t5917: F, t5920: F, t8424: F, t1378: F, t1971: F, t26036: F, t2948: F, t4579: F, t553: F, t4585: F, t8361: F) -> (F, F, F, F, F) {
    let t26341 = t2970 * t5917;
    let t26358 = t8424 * t5920;
    let t26386 = t26036 * t1378 * t1971;
    let t26399 = t2948 * t4579 * t553;
    let t26402 = t8361 * t4585 * t1971;
    (t26341, t26358, t26386, t26399, t26402)
}
