//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 858/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk858(t3418: f64, t8042: f64, t11288: f64, t2497: f64, t10800: f64, t11127: f64, t3073: f64, t44670: f64, t44671: f64, t44674: f64, t44676: f64, t44678: f64, t44679: f64, t44681: f64, t44684: f64, t44687: f64, t44689: f64, t44692: f64, t44694: f64, t44697: f64, t44702: f64, t8862: f64) -> (f64, f64, f64) {
    let t44704 = 2.0_f64 * t8042 * t3418;
    let t44705 = t11288 * t2497;
    let t44706 = -2.0_f64 * t10800 * t3073 + 4.0_f64 * t11127 * t8862 - t44670 + t44671 + t44674 - t44676 + t44678 + t44679 - t44681 + t44684 - t44687 + t44689 - t44692 + t44694 - t44697 - t44702 + t44704 + t44705;
    (t44704, t44705, t44706)
}
