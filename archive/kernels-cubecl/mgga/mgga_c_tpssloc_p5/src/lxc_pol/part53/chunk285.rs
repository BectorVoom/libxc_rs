//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 285/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk285<F: Float>(t557: F, t68: F, t1307: F, t1345: F, t546: F, t548: F) -> (F, F, F) {
    let t1347 = t68 * t557;
    let t1348 = t1347 * t1307;
    let t1351 = -t1345 * t548 + F::cast_from(3.0_f64) * t1348 * t546;
    (t1347, t1348, t1351)
}
