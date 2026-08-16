//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1838/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1838<F: Float>(t1851: F, t7240: F, t1858: F, t7222: F, t26959: F, t6495: F, t26070: F, t7032: F, t26073: F, t26076: F, t23998: F, t7435: F) -> (F, F, F, F, F, F, F) {
    let t91834 = F::cast_from(2.0_f64) * t1851 * t7240;
    let t91842 = F::cast_from(2.0_f64) * t7222 * t1858;
    let t91890 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t6495 * t26959;
    let t91894 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t26070 * t7032;
    let t91896 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t26073 * t7032;
    let t91898 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t26076 * t7032;
    let t91900 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t7435 * t23998;
    (t91834, t91842, t91890, t91894, t91896, t91898, t91900)
}
