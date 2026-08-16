//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 813/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk813<F: Float>(t1429: F, t8649: F, t1435: F, t3333: F, t444: F, t8635: F, t27: F, t23: F, t2500: F, t2504: F, t3315: F, t3319: F, t3324: F, t434: F, t445: F, t6658: F, t6679: F, t7: F, t8621: F, t8625: F, t8631: F, t8636: F, t8646: F, t980: F) -> (F, F, F, F, F) {
    let t8650 = t8649 * t1429;
    let t8653 = t1435 * t3333;
    let t8654 = t8653 * t444;
    let t8657 = -t8635;
    let t8658 = t27 * t8657;
    let t8661 = -F::cast_from(80.0_f64) / F::cast_from(27.0_f64) * t434 * t3315 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t7 * t8621 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t6658 * t8625 - F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t434 * t3319 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t7 * t8631 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7 * t8636 + F::cast_from(440.0_f64) / F::cast_from(27.0_f64) * t3324 * t445 - F::cast_from(160.0_f64) / F::cast_from(27.0_f64) * t980 * t2500 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t980 * t2504 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t23 * t8646 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t6679 * t8650 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t23 * t8654 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t23 * t8658;
    (t8650, t8654, t8657, t8658, t8661)
}
