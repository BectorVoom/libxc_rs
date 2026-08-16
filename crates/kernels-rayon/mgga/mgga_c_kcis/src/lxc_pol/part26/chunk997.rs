//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 997/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk997(t4281: f64, t7296: f64, t2042: f64, t6037: f64, t1533: f64, t1489: f64, t6917: f64, t4261: f64, t6027: f64, t17514: f64, t2055: f64, t17474: f64, t5919: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22659 = t4281 * t7296;
    let t22661 = t2042 * t6037;
    let t22662 = t1533 * t22661;
    let t22664 = t6917 * t1489;
    let t22665 = t4261 * t22664;
    let t22666 = t6027 * t22665;
    let t22668 = t17514 * t2055;
    let t22670 = t17474 * t5919;
    (t22659, t22662, t22664, t22666, t22668, t22670)
}
