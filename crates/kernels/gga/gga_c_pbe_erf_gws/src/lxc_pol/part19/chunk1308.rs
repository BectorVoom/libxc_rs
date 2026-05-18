//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1308/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1308<F: Float>(t11651: F, t338: F, t54090: F, t12084: F, t4028: F, t11915: F, t4049: F, t11981: F, t3123: F, t8897: F, t8996: F, t11542: F, t51470: F) -> (F, F, F, F, F, F, F) {
    let t56943 = t54090 * t338 * t11651;
    let t56945 = t4028 * t12084;
    let t56947 = t4049 * t11915;
    let t56949 = t4028 * t11981;
    let t56954 = t3123 * t8897;
    let t56956 = t3123 * t8996;
    let t56958 = t51470 * t11542;
    (t56943, t56945, t56947, t56949, t56954, t56956, t56958)
}
