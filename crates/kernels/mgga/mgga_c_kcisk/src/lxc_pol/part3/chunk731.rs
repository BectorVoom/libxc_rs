//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 731/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk731<F: Float>(t1417: F, t4654: F, t1889: F, t3517: F, t10660: F, t1882: F, t706: F, t1884: F, t10671: F, t677: F, t1821: F, t4663: F) -> (F, F, F, F, F, F, F) {
    let t11342 = t1417 * t4654;
    let t11344 = t3517 * t1889;
    let t11346 = t1882 * t10660;
    let t11347 = t706 * t11346;
    let t11350 = t3517 * t1884;
    let t11352 = t10671 * t677;
    let t11355 = t4663 * t1821;
    (t11342, t11344, t11346, t11347, t11350, t11352, t11355)
}
