//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 572/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk572(t4108: f64, t4115: f64, t6020: f64, t6066: f64, t7914: f64, t7917: f64, t7920: f64, t7932: f64, t7939: f64, t7945: f64, t7947: f64, t7951: f64, t7954: f64, t7957: f64) -> f64 {
    let t7993 = -0.1294625e1_f64 * t7932 + 0.258925e1_f64 * t7939 + t4108 + 0.20128333333333333334e0_f64 * t6020 - 0.20128333333333333333e0_f64 * t7914 + 0.60385e0_f64 * t7917 - 0.301925e0_f64 * t7920 + 0.82524375e-1_f64 * t7945 + 0.16504875e0_f64 * t7947 + t4115 + 0.22076e0_f64 * t6066 - 0.5519e-1_f64 * t7951 + 0.33114e0_f64 * t7954 - 0.16557e0_f64 * t7957;
    t7993
}
