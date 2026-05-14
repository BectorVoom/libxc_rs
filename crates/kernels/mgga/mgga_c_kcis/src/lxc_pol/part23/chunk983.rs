//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 983/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk983<F: Float>(t28356: F, t7924: F, t1394: F, t1489: F, t2046: F, t27387: F, t1464: F, t491: F, t5742: F, t990: F) -> (F, F, F, F, F, F) {
    let t28357 = t28356 * t7924;
    let t28358 = t1394 * t28357;
    let t28360 = t2046 * t1489;
    let t28361 = t27387 * t28360;
    let t28362 = t1464 * t28361;
    let t28369 = t5742 * t491 * t990;
    (t28357, t28358, t28360, t28361, t28362, t28369)
}
