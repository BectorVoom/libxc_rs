//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 915/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk915<F: Float>(t12044: F, t2120: F, t3550: F, t1280: F, t1982: F, t3553: F, t795: F, t1234: F, t3547: F, t1496: F, t184: F, t549: F, t813: F, t12034: F, t12039: F, t12040: F, t12041: F, t12042: F, t12043: F) -> (F, F, F, F, F, F, F, F) {
    let t12045 = 4.0 / 15.0 * t12044;
    let t12046 = t2120 * t3550;
    let t12047 = 8.0 / 45.0 * t12046;
    let t12049 = 2.0 / 5.0 * t1982 * t1280;
    let t12050 = t795 * t3553;
    let t12051 = 4.0 / 45.0 * t12050;
    let t12052 = t1982 * t1234;
    let t12053 = 8.0 / 15.0 * t12052;
    let t12055 = 2.0 / 15.0 * t795 * t3547;
    let t12059 = 4.0 / 5.0 * t549 * t1496 * t184 * t813;
    let t12060 = t12034 + t12039 + t12040 + t12041 + t12042 + t12043 - t12045 - t12047 - t12049 + t12051 - t12053 - t12055 + t12059;
    (t12045, t12047, t12049, t12051, t12053, t12055, t12059, t12060)
}
