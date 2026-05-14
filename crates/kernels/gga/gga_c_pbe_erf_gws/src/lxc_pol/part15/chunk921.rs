//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 921/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk921<F: Float>(t8859: F, t8918: F, t8955: F, t9003: F, t9034: F, t9106: F, t9139: F, t9197: F, t339: F, t338: F, t376: F, t1144: F, t2353: F, t2362: F, t2379: F, t2408: F, t3079: F, t3207: F, t335: F, t6156: F, t6173: F, t6793: F, t6797: F, t8654: F, t8776: F, t8780: F, t8784: F, t8790: F, t8793: F, t8797: F, t8803: F, t8806: F, t8810: F, t8812: F, t8818: F) -> (F, F, F) {
    let t9200 = t8859 + t8918 + t8955 + t9003 + t9034 + t9106 + t9139 + t9197;
    let t9201 = t339 * t9200;
    let t9203 = t338 * t9201 * t376;
    let t9208 = t338 * t1144 * t2353;
    let t9211 = -t8776 * t2362 / 32.0 + t8780 + t8784 * t3079 / 96.0 + t6793 * t8790 / 24.0 + t8793 * t6797 / 24.0 + t2408 * t8797 / 24.0 - t8803 + t3207 * t8806 / 8.0 - t8810 + t2408 * t8812 / 24.0 + 7.0 / 288.0 * t6156 - t8654 * t2379 / 48.0 - 35.0 / 432.0 * t8818 + t335 * t9203 / 96.0 - 7.0 / 144.0 * t6173 - t335 * t9208 / 96.0;
    (t9201, t9203, t9211)
}
