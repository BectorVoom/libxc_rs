//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1379/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1379<F: Float>(t26984: F, t7026: F, t10532: F, t10533: F, t34246: F, t1397: F, t8410: F, t21139: F, t20008: F, t544: F, t6744: F, t986: F) -> (F, F, F, F) {
    let t34466 = t26984 * t7026;
    let t34467 = F::cast_from(0.89376224879626066674e-1_f64) * t34466;
    let t34470 = F::cast_from(0.27606906686822939767e2_f64) * t10532 * t10533 * t34246;
    let t34471 = t1397 * t8410;
    let t34473 = F::cast_from(0.50050685932590597338e1_f64) * t34471 * t21139;
    let t34477 = F::cast_from(0.17875244975925213335e2_f64) * t544 * t20008 * t986 * t6744;
    (t34467, t34470, t34473, t34477)
}
