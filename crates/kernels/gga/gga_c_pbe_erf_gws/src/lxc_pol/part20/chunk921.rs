//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 921/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk921<F: Float>(t11363: F, t2409: F, t831: F, t3889: F, t840: F, t4383: F, t6158: F, t1114: F, t814: F, t9914: F, t353: F, t859: F, t1144: F, t3307: F, t338: F, t328: F, t3780: F) -> (F, F, F, F, F, F) {
    let t11365 = t2409 * t831 * t11363;
    let t11368 = t840 * t3889;
    let t11374 = t6158 * t4383;
    let t11375 = t1114 * t11374;
    let t11376 = t9914 * t814;
    let t11377 = t353 * t11376;
    let t11378 = t859 * t11377;
    let t11384 = t338 * t1144 * t3307;
    let t11387 = t3780 * t328;
    (t11365, t11368, t11375, t11378, t11384, t11387)
}
