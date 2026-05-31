//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 498/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk498<F: Float>(t203: F, t328: F, t202: F, t2607: F, t4: F, t11: F, t2: F, t39: F, t2673: F, t672: F, t210: F, t21: F, t5: F, t575: F) -> (F, F, F, F, F, F) {
    let t2676 = t203 * t328;
    let t2677 = t202 * t2676;
    let t2679 = t4 * t2607;
    let t2681 = F::cast_from(1.0_f64)/pow_3_2::<F>(t11);
    let t2682 = t2681 * t2;
    let t2683 = t2682 * t39;
    let t2685 = t672 * t2673;
    let t2687 = t210 * t2676;
    let t2690 = t21 * t5 * t575;
    (t2677, t2679, t2683, t2685, t2687, t2690)
}
