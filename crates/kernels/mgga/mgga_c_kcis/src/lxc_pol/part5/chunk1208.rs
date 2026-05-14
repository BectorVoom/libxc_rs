//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1208/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1208<F: Float>(t17450: F, t2039: F, t17505: F, t5913: F, t21804: F, t4261: F, t6027: F, t17382: F, t21910: F, t5908: F, t22349: F, t22352: F, t22355: F, t22359: F, t22362: F, t22365: F, t22367: F) -> (F, F, F, F, F) {
    let t22369 = t17450 * t2039;
    let t22371 = t17505 * t5913;
    let t22373 = t4261 * t21804;
    let t22374 = t6027 * t22373;
    let t22376 = t17382 * t21910;
    let t22377 = t5908 * t22376;
    let t22379 = -t22349 / 128.0 + t22352 / 4.0 + t22355 / 288.0 - t22359 / 16.0 + t22362 / 8.0 + t22365 / 192.0 - t22367 / 18.0 - t22369 / 8.0 - t22371 / 18.0 + t22374 / 12.0 + t22377 / 54.0;
    (t22369, t22371, t22374, t22377, t22379)
}
