//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1099/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1099<F: Float>(t10009: F, t2013: F, t10004: F, t5676: F, t1645: F, t7124: F, t23309: F, t7372: F, t1966: F, t9801: F, t5640: F, t9807: F) -> (F, F, F, F, F, F) {
    let t28378 = t2013 * t10009;
    let t28381 = F::new(0.11916829983950142223e0) * t5676 * t10004;
    let t28387 = t1645 * t7124;
    let t28406 = F::new(0.59584149919750711116e-1) * t23309 * t7372;
    let t28407 = t1966 * t9801;
    let t28409 = t5640 * t9807;
    (t28378, t28381, t28387, t28406, t28407, t28409)
}
