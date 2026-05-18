//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1230/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1230<F: Float>(t1377: F, t1593: F, t1444: F, t3717: F, t52613: F, t7908: F, t7910: F, t1598: F, t37602: F, t11418: F, t1386: F, t1466: F, t491: F) -> (F, F, F, F, F, F) {
    let t94246 = t1593 * t1377;
    let t94274 = t3717 * t1444;
    let t94287 = t7908 * t52613 * t7910;
    let t94390 = t37602 * t1598;
    let t94408 = t1386 * t11418;
    let t94424 = t1466 * t491;
    (t94246, t94274, t94287, t94390, t94408, t94424)
}
