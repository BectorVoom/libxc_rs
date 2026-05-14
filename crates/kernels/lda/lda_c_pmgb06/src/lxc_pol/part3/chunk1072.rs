//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1072/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1072<F: Float>(t5887: F, t707: F, t5891: F, t5895: F, t1770: F, t419: F, t4238: F, t794: F, t4044: F, t6007: F, t769: F, t1289: F, t4232: F, t10577: F, t4354: F, t2257: F, t4042: F) -> (F, F, F, F, F, F, F, F) {
    let t14567 = t707 * t5887;
    let t14569 = t707 * t5891;
    let t14570 = 0.11974234010254609 * t14569;
    let t14571 = t707 * t5895;
    let t14575 = t4238 * t794 * t419 * t1770;
    let t14587 = t6007 * t769 * t4044;
    let t14593 = t4232 * t769 * t1289;
    let t14596 = t10577 * t4354;
    let t14601 = t2257 * t4042;
    (t14567, t14570, t14571, t14575, t14587, t14593, t14596, t14601)
}
