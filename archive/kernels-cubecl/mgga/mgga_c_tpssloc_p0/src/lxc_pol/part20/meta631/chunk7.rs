//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2303/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2303<F: Float>(t2: F, t2756: F, t584: F, t873: F, t13501: F, t16: F, t265: F, t4331: F, t591: F, t1409: F, t41666: F, t9288: F) -> (F, F, F, F, F, F) {
    let t47668 = F::cast_from(3.0_f64) * t2756 * t2 * t584;
    let t47670 = F::cast_from(3.0_f64) * t873 * t584;
    let t47672 = F::cast_from(9.0_f64) * t13501 * t16;
    let t47674 = F::cast_from(6.0_f64) * t265 * t16;
    let t47676 = F::cast_from(12.0_f64) * t4331 * t591;
    let t47679 = t41666 * t1409 * t9288;
    (t47668, t47670, t47672, t47674, t47676, t47679)
}
