//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1210/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1210(t37674: f64, t37676: f64, t37681: f64, t37696: f64, t37700: f64, t39569: f64, t39572: f64, t39577: f64, t39579: f64, t39581: f64, t39583: f64, t39586: f64) -> f64 {
    let t41456 = 0.43663693315433241794e-2_f64 * t39569 + 0.26198215989259945076e-1_f64 * t39572 - 0.13869154784086829701e1_f64 * t37674 + 0.46230515946956099004e0_f64 * t37676 - 0.97574405393827830187e-2_f64 * t37681 + 0.10975748638225852664e0_f64 * t39577 - 0.51220160311720645767e0_f64 * t39579 + 0.34672886960217074252e0_f64 * t39581 + 0.10975748638225852664e0_f64 * t39583 + 0.86682217400542685632e-1_f64 * t39586 + 0.46574606203128791246e-1_f64 * t37696 + 0.23417857294518679246e0_f64 * t37700;
    t41456
}
