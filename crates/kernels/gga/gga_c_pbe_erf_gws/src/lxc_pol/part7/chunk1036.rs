//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1036/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1036<F: Float>(t6284: F, t6627: F, t2157: F, t2182: F, t3138: F, t6177: F, t6523: F, t20797: F, t20799: F, t20801: F, t20806: F, t20808: F, t20813: F, t20815: F, t20821: F, t2190: F, t2306: F, t2343: F, t3235: F, t6282: F, t902: F, t905: F) -> (F, F, F) {
    let t20823 = t6627 * t6284;
    let t20825 = t2157 * t2182;
    let t20829 = 3.0 / 4.0 * t3138 * t6523 * t6177 * t20825;
    let t20830 = t20797 + t20799 + t20801 - t20806 + t902 * t905 * t20808 * t2306 / 256.0 - 7.0 / 576.0 * t20813 - 7.0 / 64.0 * t20815 - t2343 * t3235 * t6282 * t2190 / 256.0 + 7.0 / 16.0 * t20821 - 7.0 / 48.0 * t20823 + t20829;
    (t20825, t20829, t20830)
}
