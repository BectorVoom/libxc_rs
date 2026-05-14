//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1089/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1089<F: Float>(t19233: F, t4093: F, t39: F, t5585: F, t1611: F, t8: F, t668: F, t771: F, t1402: F, t1771: F, t6005: F, t1403: F, t2399: F, t6063: F, t6067: F, t173: F, t24277: F) -> (F, F, F, F, F, F, F, F, F) {
    let t83349 = t19233 * t4093;
    let t92354 = t39 * t5585;
    let t93076 = t8 * t1611;
    let t96339 = t771 * t668;
    let t96360 = t1402 * t1771;
    let t96361 = t96360 * t6005;
    let t96397 = t1403 * t2399 * t6063;
    let t96400 = t1403 * t2399 * t6067;
    let t96419 = t173 * t24277;
    (t83349, t92354, t93076, t96339, t96360, t96361, t96397, t96400, t96419)
}
