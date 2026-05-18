//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 923/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk923<F: Float>(t10760: F, t13733: F, t1426: F, t17276: F, t17284: F, t17287: F, t17312: F, t2301: F, t350: F, t4009: F, t4835: F, t4846: F, t8345: F, t974: F) -> F {
    let t17314 = F::new(6.0) * t10760 * t4835 - F::new(3.0) * t13733 * t1426 + t17276 * t350 - F::new(6.0) * t8345 * t17284 + F::new(6.0) * t2301 * t17287 - t974 * t17312 - F::new(3.0) * t4009 * t4846;
    t17314
}
