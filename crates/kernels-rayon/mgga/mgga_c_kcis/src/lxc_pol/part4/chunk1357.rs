//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1357/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1357(t17453: f64, t5904: f64, t4292: f64, t16653: f64, t4293: f64, t15898: f64, t4261: f64, t4260: f64, t11825: f64, t4291: f64, t15973: f64, t6011: f64) -> (f64, f64, f64, f64, f64) {
    let t17454 = t5904 * t17453;
    let t17455 = t4292 * t17454;
    let t17457 = t4293 * t16653;
    let t17458 = t4292 * t17457;
    let t17460 = t4261 * t15898;
    let t17461 = t4260 * t17460;
    let t17463 = t11825 * t4291;
    let t17464 = t6011 * t15973;
    (t17455, t17458, t17461, t17463, t17464)
}
