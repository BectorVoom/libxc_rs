//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1210/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1210(t1851: f64, t2844: f64, t2630: f64, t11020: f64, t421: f64, t4951: f64, t13511: f64, t1662: f64, t3532: f64, t11072: f64, t3490: f64, t5299: f64) -> (f64, f64, f64, f64) {
    let t15529 = t1851 * t2844;
    let t15530 = t15529 * t2630;
    let t15531 = t11020 * t15530;
    let t15534 = t4951 * t421;
    let t15535 = t15534 * t13511;
    let t15540 = t1662 * t3532;
    let t15541 = t11072 * t15540;
    let t15547 = t3490 * t5299 / 324.0_f64;
    (t15531, t15535, t15541, t15547)
}
