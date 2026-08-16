//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2295/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2295<F: Float>(t1193: F, t27506: F, t7378: F, t11153: F, t491: F, t24660: F, t8034: F, t24667: F, t24826: F, t27537: F, t12648: F, t12652: F, t14165: F, t14985: F, t24781: F, t24784: F, t24804: F, t24806: F, t24812: F, t24816: F, t24822: F, t27406: F, t27536: F, t27549: F, t27550: F, t27551: F, t5064: F, t7373: F, t7375: F, t7376: F) -> F {
    let t94909 = t27506 * t1193;
    let t94911 = F::cast_from(0.14621636149762012769e-1_f64) * t94909 * t7378;
    let t94920 = t491 * t11153;
    let t94932 = t8034 * t24660;
    let t94936 = t8034 * t24667;
    let t94941 = F::cast_from(0.54831135561607547884e-2_f64) * t24826 * t27537;
    let t94942 = F::cast_from(0.21932454224643019153e-1_f64) * t27406 * t24781 + F::cast_from(0.82246703342411321825e-2_f64) * t7373 * t7375 * t14985 * t7376 - t94911 + F::cast_from(0.73108180748810063846e-2_f64) * t27549 * t27550 * t27551 * t12652 + F::cast_from(0.36554090374405031923e-2_f64) * t27549 * t27550 * t27551 * t12648 + F::cast_from(0.21932454224643019154e-1_f64) * t27549 * t27550 * t94920 * t14165 + t5064 * t24804 - F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t27536 * t24784 - F::cast_from(0.82246703342411321825e-2_f64) * t7373 * t27536 * t24806 - F::cast_from(0.16449340668482264365e-1_f64) * t24812 * t94932 * t24816 + F::cast_from(0.82246703342411321825e-2_f64) * t24812 * t94936 * t24822 - t94941;
    t94942
}
