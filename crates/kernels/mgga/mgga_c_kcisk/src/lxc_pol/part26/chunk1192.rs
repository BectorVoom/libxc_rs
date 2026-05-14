//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1192/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1192<F: Float>(t20: F, t394: F, t8020: F, t1220: F, t7744: F, t9447: F, t1312: F, t7736: F, t3952: F, t2213: F, t2236: F, t415: F, t5625: F, t7706: F, t9461: F, t1339: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t34794 = t8020 * t394 * t20;
    let t34795 = t1220 * t34794;
    let t34798 = t9447 * t7744;
    let t34799 = t1312 * t34798;
    let t34802 = t9447 * t7736;
    let t34803 = t3952 * t34802;
    let t34806 = t2213 * t2236;
    let t34807 = t415 * t34806;
    let t34809 = t5625 * t7706;
    let t34810 = t9461 * t34809;
    let t34811 = t1339 * t34810;
    (t34794, t34795, t34798, t34799, t34802, t34803, t34806, t34807, t34809, t34810, t34811)
}
