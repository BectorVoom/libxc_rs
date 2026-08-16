//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 479/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk479(t3806: f64, t1609: f64, t554: f64, t551: f64, t3517: f64, t710: f64, t579: f64, t695: f64) -> (f64, f64, f64, f64, f64) {
    let t4519 = 0.38691203703703703703e-3_f64 * t3806;
    let t4534 = 1.0_f64 / t1609 / t554;
    let t4535 = t551 * t4534;
    let t4586 = 0.21901432222222222222e-3_f64 * t3517 * t710;
    let t4593 = t579 * t695;
    let t4594 = 1.0_f64 / t4593;
    (t4519, t4534, t4535, t4586, t4594)
}
