//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 961/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk961<F: Float>(t751: F, t8269: F, t5942: F, t8424: F, t2970: F, t5931: F, t5927: F, t3013: F, t671: F, t1049: F, t1985: F, t5904: F) -> (F, F, F, F, F, F, F) {
    let t26153 = t751 * t8269;
    let t26196 = t8424 * t5942;
    let t26204 = t2970 * t5931;
    let t26242 = t2970 * t5927;
    let t26308 = t3013 * t671;
    let t26314 = t1049 * t1985;
    let t26328 = t1049 * t5904;
    (t26153, t26196, t26204, t26242, t26308, t26314, t26328)
}
