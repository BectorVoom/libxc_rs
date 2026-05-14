//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1256/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1256<F: Float>(t22299: F, t22301: F, t22303: F, t22305: F, t22307: F, t22309: F, t22312: F, t22315: F, t22316: F, t22319: F, t22715: F, t187: F, t23375: F, t449: F, t446: F, t2132: F, t5407: F) -> (F, F) {
    let t23376 = -t22299 + t22301 + t22303 - t22305 + t22307 - t22309 + t22312 - t22315 + t22316 - t22319 + t22715;
    let t23379 = t22299 - t22301 - t22303 + t22305 - t22307 + t22309 - t22312 + t22315 - t22316 + t22319 - t22715 + t187 * (t23375 + t23376);
    let t23380 = t449 * t23379;
    let t23381 = t446 * t23380;
    let t23383 = t5407 * t2132;
    (t23381, t23383)
}
