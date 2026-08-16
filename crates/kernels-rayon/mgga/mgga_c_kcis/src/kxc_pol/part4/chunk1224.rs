//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1224/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1224(t15793: f64, t236: f64, t233: f64, t1885: f64, t4504: f64, t446: f64, t12274: f64, t2003: f64, t1396: f64, t531: f64, t1395: f64, t5780: f64) -> (f64, f64, f64, f64) {
    let t15794 = t236 * t15793;
    let t15795 = t233 * t15794;
    let t15797 = t1885 * t4504;
    let t15798 = t446 * t15797;
    let t15800 = t12274 * t2003;
    let t15802 = t1396 * t531;
    let t15803 = t1395 * t15802;
    let t15804 = t5780 * t15803;
    (t15795, t15798, t15800, t15804)
}
