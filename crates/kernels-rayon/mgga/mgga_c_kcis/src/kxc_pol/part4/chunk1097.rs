//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1097/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1097(t13802: f64, t949: f64, t2986: f64, t3031: f64, t4758: f64, t4764: f64, t10974: f64, t4763: f64, t1692: f64, t9630: f64, t3006: f64, t9634: f64) -> (f64, f64, f64, f64, f64) {
    let t13803 = t13802 * t949;
    let t13805 = 0.32163648644302209644e2_f64 * t2986 * t13803;
    let t13806 = t3031 * t4758;
    let t13807 = t13806 * t4764;
    let t13812 = t4763 * t10974;
    let t13817 = t9630 * t1692;
    let t13818 = t9634 * t3006;
    (t13805, t13807, t13812, t13817, t13818)
}
