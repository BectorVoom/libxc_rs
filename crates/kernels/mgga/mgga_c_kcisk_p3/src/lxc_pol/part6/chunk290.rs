//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 290/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk290<F: Float>(t1203: F, t325: F, t240: F, t547: F, t524: F) -> (F, F, F, F, F) {
    let t1542 = t325 * t1203;
    let t1550 = t240 * t325;
    let t1555 = t547 * t547;
    let t1556 = F::cast_from(1.0_f64) / t1555;
    let t1557 = t524 * t1556;
    (t1542, t1550, t1555, t1556, t1557)
}
