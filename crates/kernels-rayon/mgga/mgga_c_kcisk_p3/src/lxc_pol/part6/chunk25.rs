//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 25/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk25(t12: f64, t15: f64, t18: f64, t26: f64) -> (f64, f64, f64) {
    let t52 = 0.51785e1_f64 * t15 + 0.905775e0_f64 * t12 + 0.1100325e0_f64 * t18 + 0.1241775e0_f64 * t26;
    let t55 = 1.0_f64 + 0.29608574643216675549e2_f64 / t52;
    let t56 = f64::ln(t55);
    (t52, t55, t56)
}
