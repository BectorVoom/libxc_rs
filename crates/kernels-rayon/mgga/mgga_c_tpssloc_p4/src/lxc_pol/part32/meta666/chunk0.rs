//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2098/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2098(t1193: f64, t27506: f64, t7378: f64, t11153: f64, t491: f64, t24826: f64, t27537: f64, t27526: f64, t86094: f64, t24660: f64, t24850: f64, t24667: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94909 = t27506 * t1193;
    let t94911 = 0.14621636149762012769e-1_f64 * t94909 * t7378;
    let t94920 = t491 * t11153;
    let t94941 = 0.54831135561607547884e-2_f64 * t24826 * t27537;
    let t94947 = 0.18277045187202515961e-2_f64 * t86094 * t27526;
    let t94948 = t24660 * t24850;
    let t94954 = t24667 * t24850;
    (t94909, t94911, t94920, t94941, t94947, t94948, t94954)
}
