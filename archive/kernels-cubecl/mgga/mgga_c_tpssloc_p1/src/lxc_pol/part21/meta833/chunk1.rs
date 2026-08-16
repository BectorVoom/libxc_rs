//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2942/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2942<F: Float>(t10255: F, t17800: F, t17804: F, t2986: F, t42830: F, t42962: F, t42968: F, t4510: F, t5821: F, t59715: F, t61245: F, t61252: F, t61258: F, t61261: F, t61264: F, t61273: F) -> F {
    let t61275 = F::cast_from(0.24691358024691358024e-3_f64) * t61245 + F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t17804 * t10255 - F::cast_from(0.18518518518518518518e-3_f64) * t61252 + F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t17800 * t10255 + F::cast_from(0.49382716049382716048e-3_f64) * t61258 + F::cast_from(0.5761316872427983539e-3_f64) * t61261 - F::cast_from(0.24691358024691358024e-3_f64) * t61264 - F::cast_from(0.22222222222222222221e-2_f64) * t2986 * t4510 * t59715 + F::cast_from(0.12345679012345679012e-3_f64) * t42962 + F::cast_from(0.32921810699588477366e-3_f64) * t42968 - F::cast_from(0.54320987654320987651e-2_f64) * t42830 * t5821 + F::cast_from(0.98765432098765432095e-3_f64) * t61273;
    t61275
}
