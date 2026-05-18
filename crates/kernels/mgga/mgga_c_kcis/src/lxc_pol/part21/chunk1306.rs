//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1306/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1306<F: Float>(t26854: F, t8030: F, t1014: F, t27931: F, t26796: F, t303: F, t5019: F, t27964: F, t7699: F, t26732: F, t26742: F, t26784: F, t27919: F, t7696: F, t8034: F, t92981: F, t92991: F, t92993: F, t92997: F) -> (F, F, F) {
    let t96015 = t8030 * t26854;
    let t96018 = t1014 * t27931;
    let t96019 = F::new(0.33163888888888888888e-2) * t96018;
    let t96021 = t303 * t26796 * t5019;
    let t96026 = F::new(0.12356481481481481482e-2) * t27964 * t7699;
    let t96034 = -F::new(0.13901041666666666667e-2) * t8030 * t26784 + F::new(0.15445601851851851852e-3) * t96015 - F::new(0.58958024691358024689e-2) * t92981 - t96019 + F::new(0.13265555555555555555e-1) * t96021 + F::new(0.67960648148148148147e-2) * t26742 * t8034 + t96026 + F::new(0.11054629629629629629e-2) * t92991 + F::new(0.11054629629629629629e-2) * t92993 - F::new(0.73697530864197530861e-3) * t92997 + F::new(0.69505208333333333333e-3) * t8030 * t26732 - F::new(0.37069444444444444444e-2) * t7696 * t27919;
    (t96018, t96021, t96034)
}
