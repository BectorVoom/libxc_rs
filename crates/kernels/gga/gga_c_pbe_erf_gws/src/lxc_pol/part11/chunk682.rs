//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 682/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk682<F: Float>(t142: F, t3644: F, t525: F, t3772: F, t817: F, t2365: F, t3747: F, t1114: F, t833: F, t3889: F, t840: F, t4383: F, t6158: F, t328: F, t3780: F, t1105: F, t1134: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11299 = t142 * t3644;
    let t11300 = t525 * t11299;
    let t11318 = t3772 * t817;
    let t11347 = t3747 * t2365;
    let t11348 = t1114 * t11347;
    let t11349 = t11348 * t833;
    let t11368 = t840 * t3889;
    let t11374 = t6158 * t4383;
    let t11375 = t1114 * t11374;
    let t11387 = t3780 * t328;
    let t11412 = t1134 * t1105;
    (t11299, t11300, t11318, t11347, t11348, t11349, t11368, t11375, t11387, t11412)
}
