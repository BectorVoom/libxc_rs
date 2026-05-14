//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 466/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk466<F: Float>(t209: F, t589: F, t1501: F, t221: F, t205: F, t6017: F, t23: F, t600: F, t1839: F, t4388: F, t446: F, t1392: F, t1487: F, t1156: F, t1835: F, t472: F, t6067: F) -> (F, F, F, F, F, F, F) {
    let t6213 = t209 * t589;
    let t6215 = t221 * t1501 * t6213;
    let t6218 = t6017 * t205;
    let t6224 = t600 * t23;
    let t6231 = t4388 * t1839;
    let t6232 = t6231 * t446;
    let t6235 = t1487 * t1392;
    let t6240 = t1156 * t1835;
    let t6241 = t6240 * t446;
    let t6244 = t472 * t6067;
    (t6215, t6218, t6224, t6232, t6235, t6241, t6244)
}
