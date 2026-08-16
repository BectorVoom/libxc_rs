//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2295/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2295(t1193: f64, t27506: f64, t7378: f64, t11153: f64, t491: f64, t24660: f64, t8034: f64, t24667: f64, t24826: f64, t27537: f64, t12648: f64, t12652: f64, t14165: f64, t14985: f64, t24781: f64, t24784: f64, t24804: f64, t24806: f64, t24812: f64, t24816: f64, t24822: f64, t27406: f64, t27536: f64, t27549: f64, t27550: f64, t27551: f64, t5064: f64, t7373: f64, t7375: f64, t7376: f64) -> f64 {
    let t94909 = t27506 * t1193;
    let t94911 = 0.14621636149762012769e-1_f64 * t94909 * t7378;
    let t94920 = t491 * t11153;
    let t94932 = t8034 * t24660;
    let t94936 = t8034 * t24667;
    let t94941 = 0.54831135561607547884e-2_f64 * t24826 * t27537;
    let t94942 = 0.21932454224643019153e-1_f64 * t27406 * t24781 + 0.82246703342411321825e-2_f64 * t7373 * t7375 * t14985 * t7376 - t94911 + 0.73108180748810063846e-2_f64 * t27549 * t27550 * t27551 * t12652 + 0.36554090374405031923e-2_f64 * t27549 * t27550 * t27551 * t12648 + 0.21932454224643019154e-1_f64 * t27549 * t27550 * t94920 * t14165 + t5064 * t24804 - 0.16449340668482264365e-1_f64 * t7373 * t27536 * t24784 - 0.82246703342411321825e-2_f64 * t7373 * t27536 * t24806 - 0.16449340668482264365e-1_f64 * t24812 * t94932 * t24816 + 0.82246703342411321825e-2_f64 * t24812 * t94936 * t24822 - t94941;
    t94942
}
