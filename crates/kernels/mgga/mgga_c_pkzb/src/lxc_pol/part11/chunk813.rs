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
    let t8661 = -F::new(80.0) / F::new(27.0) * t434 * t3315 - F::new(10.0) / F::new(27.0) * t7 * t8621 + F::new(20.0) / F::new(9.0) * t6658 * t8625 - F::new(40.0) / F::new(9.0) * t434 * t3319 + F::new(10.0) / F::new(9.0) * t7 * t8631 + F::new(5.0) / F::new(3.0) * t7 * t8636 + F::new(440.0) / F::new(27.0) * t3324 * t445 - F::new(160.0) / F::new(27.0) * t980 * t2500 + F::new(80.0) / F::new(9.0) * t980 * t2504 - F::new(10.0) / F::new(27.0) * t23 * t8646 - F::new(20.0) / F::new(9.0) * t6679 * t8650 + F::new(10.0) / F::new(9.0) * t23 * t8654 + F::new(5.0) / F::new(3.0) * t23 * t8658;
    (t8650, t8654, t8657, t8658, t8661)
}
