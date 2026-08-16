//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 567/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk567(t4753: f64, t600: f64, t1670: f64, t45: f64, t1675: f64, t596: f64, t1683: f64) -> (f64, f64, f64, f64) {
    let t4754 = t4753 * t600;
    let t4757 = t45 * t1670;
    let t4760 = t1675 * t596;
    let t4761 = 1.0_f64 / t4760;
    let t4762 = t1683 * t1683;
    (t4754, t4757, t4761, t4762)
}
