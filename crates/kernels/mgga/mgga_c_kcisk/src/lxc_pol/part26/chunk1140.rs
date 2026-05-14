//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1140/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1140<F: Float>(t1220: F, t33520: F, t19972: F, t2714: F, t6221: F, t9433: F, t1333: F, t9824: F, t468: F, t5981: F, t415: F, t9821: F, t5621: F, t9461: F, t1339: F, t5627: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33521 = t1220 * t33520;
    let t33524 = t19972 * t2714;
    let t33527 = t6221 * t9433;
    let t33530 = t1333 * t9824;
    let t33532 = t468 * t5981;
    let t33533 = t415 * t33532;
    let t33535 = t1333 * t9821;
    let t33541 = t9461 * t5621;
    let t33542 = t1339 * t33541;
    let t33544 = t9461 * t5627;
    (t33521, t33524, t33527, t33530, t33532, t33533, t33535, t33541, t33542, t33544)
}
