//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1017/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1017<F: Float>(t529: F, t1287: F, t13778: F, t13785: F, t15016: F, t15032: F, t1558: F, t382: F, t4144: F, t4148: F, t4354: F, t525: F, t526: F, t6442: F) -> F {
    let t530 = t529 < -F::new(0.66725e-1);
    let t15039 = piecewise3::<F>(t530, F::new(0.0), F::new(10.0) / F::new(9.0) * t525 * t15016 * t382 - F::new(10.0) / F::new(9.0) * t525 * t4354 * t1287 + F::new(40.0) / F::new(27.0) * t525 * t1558 * t4144 - F::new(10.0) / F::new(9.0) * t525 * t1558 * t4148 - F::new(280.0) / F::new(243.0) * t525 * t526 * t13778 + F::new(40.0) / F::new(27.0) * t6442 * t15032 - F::new(10.0) / F::new(27.0) * t525 * t526 * t13785);
    t15039
}
