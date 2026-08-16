//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1224/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1224<F: Float>(t97782: F, t97785: F, t97787: F, t97789: F, t97791: F, t97794: F, t97796: F, t97798: F, t97802: F, t97805: F, t97807: F, t97809: F, t97811: F, t97813: F, t97815: F, t97817: F, t97819: F, t97822: F) -> F {
    let t97958 = t97782 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t97785 + t97787 / F::cast_from(96.0_f64) + t97789 / F::cast_from(128.0_f64) + t97791 / F::cast_from(96.0_f64) + t97794 / F::cast_from(6.0_f64) + t97796 / F::cast_from(54.0_f64) - t97798 / F::cast_from(36.0_f64) - t97802 / F::cast_from(9.0_f64) - t97805 / F::cast_from(3.0_f64) + t97807 / F::cast_from(48.0_f64) - t97809 / F::cast_from(6.0_f64) + t97811 / F::cast_from(6.0_f64) + t97813 / F::cast_from(24.0_f64) - t97815 / F::cast_from(16.0_f64) - t97817 / F::cast_from(32.0_f64) - t97819 / F::cast_from(12.0_f64) + t97822 / F::cast_from(36.0_f64);
    t97958
}
