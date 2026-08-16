//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1463/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1463<F: Float>(t4300: F, t865: F, t2718: F, t2684: F, t4180: F, t4181: F, t9646: F, t9647: F, t2633: F, t2645: F, t4248: F, t1496: F, t9541: F) -> (F, F, F, F, F, F) {
    let t13071 = t4300 * t865;
    let t13072 = t2718 * t13071;
    let t13076 = t4180 * t4181 * t2684;
    let t13080 = t9646 * t4181 * t9647;
    let t13084 = t2645 * t4248 * t2633;
    let t13087 = t9541 * t1496;
    (t13071, t13072, t13076, t13080, t13084, t13087)
}
