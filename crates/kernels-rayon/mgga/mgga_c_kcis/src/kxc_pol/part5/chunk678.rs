//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 678/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk678(t1064: f64, t4621: f64, t1646: f64, t331: f64, t3160: f64, t1071: f64, t822: f64, t821: f64, t9: f64, t7: f64, t118: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4866 = t1064 * t4621;
    let t4869 = t331 * t1646;
    let t4871 = t3160 * t1646;
    let t4875 = t822 * t1071;
    let t4879 = 1.0_f64 / t9 / t821;
    let t4880 = t7 * t4879;
    let t4881 = t118 * t4880;
    (t4866, t4869, t4871, t4875, t4879, t4880, t4881)
}
