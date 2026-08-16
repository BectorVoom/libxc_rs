//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1217/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1217(t1856: f64, t1899: f64, t2783: f64, t2786: f64, t5737: f64, t5802: f64, t1084: f64, t5776: f64, t1083: f64, t17577: f64, t17579: f64, t5585: f64, t7411: f64) -> (f64, f64, f64, f64, f64) {
    let t21236 = 18.0_f64 * t1899 * t2783 * t1856;
    let t21239 = 0.57895126195293126241e3_f64 * t5802 * t2786 * t5737;
    let t21251 = 24.0_f64 * t5776 * t1084 * t5737;
    let t21255 = 0.24955700379505800916e5_f64 * t17577 * t1083 * t17579 * t5737;
    let t21257 = 0.48245938496077605201e2_f64 * t7411 * t5585;
    (t21236, t21239, t21251, t21255, t21257)
}
