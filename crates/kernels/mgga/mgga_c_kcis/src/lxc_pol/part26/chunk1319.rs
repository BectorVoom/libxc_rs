//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1319/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1319<F: Float>(t20998: F, t4160: F, t94425: F, t21003: F, t98530: F, t21792: F, t2243: F, t303: F, t1928: F, t2050: F, t1394: F, t7924: F) -> (F, F, F, F) {
    let t102626 = t4160 * t94425 * t20998;
    let t102629 = t4160 * t98530 * t21003;
    let t102632 = t303 * t21792 * t2243;
    let t102634 = t2050 * t1928;
    let t102636 = t1394 * t102634 * t7924;
    (t102626, t102629, t102632, t102636)
}
