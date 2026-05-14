//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1162/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1162<F: Float>(t26984: F, t7026: F, t10532: F, t10533: F, t34246: F, t1397: F, t8410: F, t21139: F, t20008: F, t544: F, t6744: F, t986: F, t34411: F, t6716: F, t6717: F, t10409: F, t31356: F) -> (F, F, F, F, F, F) {
    let t34466 = t26984 * t7026;
    let t34467 = 0.89376224879626066674e-1 * t34466;
    let t34470 = 0.27606906686822939767e2 * t10532 * t10533 * t34246;
    let t34471 = t1397 * t8410;
    let t34473 = 0.50050685932590597338e1 * t34471 * t21139;
    let t34477 = 0.17875244975925213335e2 * t544 * t20008 * t986 * t6744;
    let t34484 = 0.69017266717057349418e1 * t6716 * t6717 * t34411;
    let t34485 = t31356 * t10409;
    (t34467, t34470, t34473, t34477, t34484, t34485)
}
