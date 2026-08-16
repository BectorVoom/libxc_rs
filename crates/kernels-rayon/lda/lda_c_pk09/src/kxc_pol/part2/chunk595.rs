//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 595/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk595(t151: f64, t2983: f64, t192: f64, t3557: f64, t179: f64, t205: f64, t3553: f64, t200: f64, t2971: f64, t830: f64, t3194: f64, t2974: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4684 = t151 * t2983;
    let t4689 = 2.460083242092564_f64 * t192 * t3557;
    let t4692 = 20.705842241814405_f64 * t179 * t3557;
    let t4694 = 3.6857207583175526_f64 * t205 * t3553;
    let t4702 = 2.6972402168825864_f64 * t200 * t3557;
    let t4705 = t830 * t2971;
    let t4706 = t4705 * t3194;
    let t4708 = t4705 * t2974;
    (t4684, t4689, t4692, t4694, t4702, t4705, t4706, t4708)
}
