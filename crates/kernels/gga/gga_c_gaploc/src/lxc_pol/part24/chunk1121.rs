//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1121/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1121<F: Float>(t29439: F, t9649: F, t123: F, t23092: F, t2563: F, t9647: F, t1841: F, t9752: F, t1843: F, t21456: F, t7064: F, t21461: F) -> (F, F, F, F, F) {
    let t29441 = F::cast_from(0.3845263115071112142e-2_f64) * t29439 * t9649;
    let t29445 = F::cast_from(0.3845263115071112142e-2_f64) * t9647 * t23092 * t123 * t2563;
    let t29447 = F::cast_from(0.17090058289204942853e-2_f64) * t1841 * t9752;
    let t29450 = F::cast_from(0.1281754371690370714e-2_f64) * t7064 * t1843 * t21456;
    let t29453 = F::cast_from(0.64087718584518535698e-3_f64) * t7064 * t1843 * t21461;
    (t29441, t29445, t29447, t29450, t29453)
}
