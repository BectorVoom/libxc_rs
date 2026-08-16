//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1329/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1329<F: Float>(t13042: F, t13053: F, t13065: F, t13463: F, t1528: F, t17083: F, t17087: F, t17090: F, t17092: F, t17095: F, t17098: F, t17100: F, t259: F, t2597: F, t4268: F, t4273: F, t5658: F, t866: F) -> F {
    let t17108 = -F::cast_from(2.0_f64) * t13042 * t1528 - F::cast_from(2.0_f64) * t13053 * t1528 - F::cast_from(2.0_f64) * t13065 * t1528 - F::cast_from(2.0_f64) * t13463 * t1528 + t17083 * t259 + F::cast_from(2.0_f64) * t17087 * t259 - t17090 * t866 - F::cast_from(2.0_f64) * t17092 * t866 + F::cast_from(2.0_f64) * t17095 * t259 + t17098 * t259 + t17100 * t259 - t2597 * t5658 + F::cast_from(4.0_f64) * t4268 * t4273;
    t17108
}
