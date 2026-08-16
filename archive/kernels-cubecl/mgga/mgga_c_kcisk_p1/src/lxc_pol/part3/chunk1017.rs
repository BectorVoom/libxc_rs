//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1017/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1017<F: Float>(t529: F, t1287: F, t13778: F, t13785: F, t15016: F, t15032: F, t1558: F, t382: F, t4144: F, t4148: F, t4354: F, t525: F, t526: F, t6442: F) -> F {
    let t530 = t529 < -F::cast_from(0.66725e-1_f64);
    let t15039 = piecewise3::<F>(t530, F::cast_from(0.0_f64), F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t525 * t15016 * t382 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t525 * t4354 * t1287 + F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t525 * t1558 * t4144 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t525 * t1558 * t4148 - F::cast_from(280.0_f64) / F::cast_from(243.0_f64) * t525 * t526 * t13778 + F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t6442 * t15032 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t525 * t526 * t13785);
    t15039
}
