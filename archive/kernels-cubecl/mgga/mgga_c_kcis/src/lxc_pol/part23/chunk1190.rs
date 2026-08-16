//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1190/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1190<F: Float>(t1014: F, t27399: F, t27447: F, t7904: F, t2237: F, t2238: F, t737: F, t27419: F, t27476: F, t61287: F, t7907: F) -> (F, F, F, F, F, F) {
    let t94594 = t1014 * t27399;
    let t94602 = t27447 * t7904;
    let t94614 = F::cast_from(0.25742669753086419753e-3_f64) * t2237 * t737 * t2238;
    let t94621 = t1014 * t27419;
    let t94624 = t1014 * t27476;
    let t94626 = t7907 * t61287;
    (t94594, t94602, t94614, t94621, t94624, t94626)
}
