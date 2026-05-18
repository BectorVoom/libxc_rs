//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1338/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1338<F: Float>(t10760: F, t13733: F, t1426: F, t17284: F, t17287: F, t17312: F, t2301: F, t23708: F, t30408: F, t350: F, t39623: F, t4009: F, t4835: F, t4846: F, t49417: F, t58056: F, t58067: F, t58080: F, t58093: F, t58109: F, t58115: F, t58132: F, t58143: F, t58156: F, t58169: F, t8345: F, t974: F) -> F {
    let t58173 = (t58056 + t58067 + t58080 + t58093) * t350 - F::new(4.0) * t49417 * t1426 + F::new(12.0) * t39623 * t4835 - F::new(6.0) * t13733 * t4846 - F::new(24.0) * t30408 * t17284 + F::new(24.0) * t10760 * t17287 - F::new(4.0) * t4009 * t17312 + F::new(24.0) * t23708 * t58109 - F::new(36.0) * t8345 * t4835 * t4846 + F::new(6.0) * t2301 * t58115 + F::new(8.0) * t2301 * t1426 * t17312 - t974 * (t58132 + t58143 + t58156 + t58169);
    t58173
}
