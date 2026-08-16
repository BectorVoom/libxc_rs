//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 990/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk990<F: Float>(t13669: F, t30326: F, t6043: F, t7938: F, t6059: F, t1248: F, t1249: F, t30273: F, t30294: F, t4065: F, t13607: F, t30233: F) -> (F, F, F, F, F, F) {
    let t30353 = t13669 * t30326;
    let t30355 = t6043 * t7938;
    let t30357 = t6059 * t7938;
    let t30360 = t1248 * t1249 * t30273;
    let t30363 = t1248 * t4065 * t30294;
    let t30366 = t1248 * t13607 * t30233;
    (t30353, t30355, t30357, t30360, t30363, t30366)
}
