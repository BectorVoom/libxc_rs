//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1088/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1088<F: Float>(t4104: F, t779: F, t238: F, t242: F, t10547: F, t226: F, t10591: F, t10593: F, t10598: F, t10602: F, t10605: F, t6614: F, t6616: F, t8706: F, t8846: F, t8847: F) -> (F, F, F, F, F) {
    let t10607 = t779 * t4104;
    let t10609 = t238 * t242 * t10607;
    let t10611 = t226 * t10547;
    let t10613 = t238 * t242 * t10611;
    let t10615 = F::cast_from(0.15358125e0_f64) * t10591 + F::cast_from(0.3071625e0_f64) * t10593 - t6614 + F::cast_from(0.27385555555555555556e0_f64) * t6616 + F::cast_from(0.5477111111111111111e0_f64) * t8706 - t8846 - t8847 - F::cast_from(0.16431333333333333333e0_f64) * t10598 + F::cast_from(0.49294e0_f64) * t10602 - F::cast_from(0.16431333333333333333e0_f64) * t10605 + F::cast_from(0.24647e0_f64) * t10609 + F::cast_from(0.24647e0_f64) * t10613;
    (t10607, t10609, t10611, t10613, t10615)
}
