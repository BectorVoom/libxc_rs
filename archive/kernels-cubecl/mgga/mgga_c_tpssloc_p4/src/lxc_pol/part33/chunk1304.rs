//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1304/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1304<F: Float>(t1858: F, t7758: F, t2029: F, t6470: F, t1851: F, t7774: F, t1390: F, t20416: F, t1983: F, t6878: F, t20085: F, t7753: F) -> (F, F, F, F, F) {
    let t100949 = t7758 * t1858;
    let t100952 = t6470 * t2029;
    let t100960 = t1851 * t7774;
    let t105159 = t1390 * t20416;
    let t105162 = F::cast_from(3.0_f64) * t1983 * t6878 * t105159;
    let t105165 = F::cast_from(6.0_f64) * t1983 * t7753 * t20085;
    (t100949, t100952, t100960, t105162, t105165)
}
