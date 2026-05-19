//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 996/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk996<F: Float>(t13399: F, t13385: F, t13387: F, t13389: F, t13392: F, t13397: F, t13404: F, t13409: F, t13413: F, t13417: F, t13420: F, t13426: F) -> F {
    let t14665 = F::cast_from(0.51588271604938271604e-3_f64) * t13399;
    let t14672 = -F::cast_from(0.52233124999999999998e-2_f64) * t13385 + F::cast_from(0.69644166666666666665e-2_f64) * t13387 + F::cast_from(0.18571777777777777777e-1_f64) * t13389 - F::cast_from(0.11607361111111111111e-2_f64) * t13392 - F::cast_from(0.11607361111111111111e-2_f64) * t13397 + t14665 - F::cast_from(0.51072388888888888887e-1_f64) * t13404 + F::cast_from(0.34048259259259259259e-1_f64) * t13409 - F::cast_from(0.18571777777777777778e-1_f64) * t13413 - F::cast_from(0.92858888888888888888e-2_f64) * t13417 + F::cast_from(0.34822083333333333333e-2_f64) * t13420 + F::cast_from(0.11607361111111111111e-2_f64) * t13426;
    t14672
}
