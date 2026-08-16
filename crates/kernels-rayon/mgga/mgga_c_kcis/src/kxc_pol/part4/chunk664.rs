//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 664/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk664(t1307: f64, t1419: f64, t3766: f64, t1444: f64, t544: f64, t1471: f64, t2642: f64, t1472: f64, t2645: f64, t1317: f64, t1319: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3767 = t1307 * t1419;
    let t3768 = t3766 * t3767;
    let t3771 = t544 * t1444;
    let t3773 = t1471 * t3771 * t2642;
    let t3777 = t1471 * t1472 * t2645;
    let t3780 = t1317 * t544;
    let t3781 = t1319 * t1319;
    (t3767, t3768, t3773, t3777, t3780, t3781)
}
