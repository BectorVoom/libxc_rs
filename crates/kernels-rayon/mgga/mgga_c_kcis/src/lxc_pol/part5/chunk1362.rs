//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1362/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1362(t1538: f64, t7385: f64, t571: f64, t1551: f64, t7328: f64, t578: f64, t5929: f64, t6002: f64, t1547: f64, t1546: f64, t21876: f64, t6011: f64) -> (f64, f64, f64, f64, f64) {
    let t22419 = t7385 * t1538;
    let t22420 = t571 * t22419;
    let t22422 = t7328 * t1551;
    let t22423 = t578 * t22422;
    let t22425 = t6002 * t5929;
    let t22427 = t7328 * t1547;
    let t22428 = t1546 * t22427;
    let t22430 = t6011 * t21876;
    (t22420, t22423, t22425, t22428, t22430)
}
