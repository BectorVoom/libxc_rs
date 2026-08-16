//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1028/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1028(t9366: f64, t9369: f64, t3556: f64, t795: f64, t2120: f64, t3550: f64, t1280: f64, t1982: f64, t3553: f64, t1234: f64, t3547: f64, t1496: f64, t184: f64, t549: f64, t813: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12042 = 8.0_f64 / 15.0_f64 * t9366;
    let t12043 = 8.0_f64 / 45.0_f64 * t9369;
    let t12044 = t795 * t3556;
    let t12045 = 4.0_f64 / 15.0_f64 * t12044;
    let t12046 = t2120 * t3550;
    let t12047 = 8.0_f64 / 45.0_f64 * t12046;
    let t12049 = 2.0_f64 / 5.0_f64 * t1982 * t1280;
    let t12050 = t795 * t3553;
    let t12051 = 4.0_f64 / 45.0_f64 * t12050;
    let t12052 = t1982 * t1234;
    let t12053 = 8.0_f64 / 15.0_f64 * t12052;
    let t12055 = 2.0_f64 / 15.0_f64 * t795 * t3547;
    let t12059 = 4.0_f64 / 5.0_f64 * t549 * t1496 * t184 * t813;
    (t12042, t12043, t12045, t12047, t12049, t12051, t12053, t12055, t12059)
}
