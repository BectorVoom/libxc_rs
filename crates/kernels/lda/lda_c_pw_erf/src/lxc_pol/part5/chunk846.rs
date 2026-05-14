//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 846/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk846<F: Float>(t1765: F, t2942: F, t1070: F, t1775: F, t1067: F, t1799: F, t8197: F, t8199: F, t8204: F, t8206: F, t339: F, t4405: F, t1034: F, t1798: F, t40: F, t3153: F, t748: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11335 = t1765 * t2942;
    let t11337 = t1070 * t1775;
    let t11338 = 96.0 * t11337;
    let t11339 = t1067 * t1799;
    let t11340 = 36.0 * t11339;
    let t11341 = 480.0 * t8197;
    let t11342 = 144.0 * t8199;
    let t11343 = 240.0 * t8204;
    let t11344 = 12.0 * t8206;
    let t11348 = t339 * t4405;
    let t11349 = 12.0 * t11348;
    let t11359 = t40 * t1798 * t1034;
    let t11360 = 3.0 * t11359;
    let t11362 = t40 * t748 * t3153;
    (t11335, t11338, t11340, t11341, t11342, t11343, t11344, t11349, t11360, t11362)
}
