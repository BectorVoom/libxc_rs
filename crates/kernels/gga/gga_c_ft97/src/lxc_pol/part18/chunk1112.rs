//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1112/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1112<F: Float>(t22914: F, t22932: F, t108: F, t22862: F, t1286: F, t1309: F, t7943: F, t22494: F, t376: F, t23133: F, t5498: F, t22885: F, t22874: F, t1637: F, t5623: F, t22904: F) -> (F, F, F, F, F, F, F, F, F) {
    let t93927 = t22914 * t22932;
    let t93931 = t22862 * t108;
    let t93946 = 14.0 / 81.0 * t1286 * t7943 * t1309;
    let t93968 = t1286 * t376 * t22494;
    let t94002 = t23133 * t5498;
    let t94019 = t1286 * t376 * t22885;
    let t94021 = t1286 * t376 * t22874;
    let t94024 = t1286 * t1637 * t5623;
    let t94026 = t22914 * t22904;
    (t93927, t93931, t93946, t93968, t94002, t94019, t94021, t94024, t94026)
}
