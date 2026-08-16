//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1045/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1045<F: Float>(t11068: F, t2617: F, t7803: F, t41281: F, t41283: F, t41286: F, t41290: F, t41293: F, t41305: F, t41307: F, t13016: F, t8478: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43881 = t7803 * t11068 * t2617;
    let t43882 = F::cast_from(0.76685851907841499353e0_f64) * t43881;
    let t43883 = F::cast_from(0.29792074959875355558e-1_f64) * t41281;
    let t43884 = F::cast_from(0.29792074959875355558e-1_f64) * t41283;
    let t43885 = F::cast_from(0.29792074959875355558e-1_f64) * t41286;
    let t43886 = F::cast_from(0.29792074959875355558e-1_f64) * t41290;
    let t43887 = F::cast_from(0.59584149919750711116e-1_f64) * t41293;
    let t43890 = F::cast_from(0.59584149919750711116e-1_f64) * t41305;
    let t43891 = F::cast_from(0.89376224879626066674e-1_f64) * t41307;
    let t43895 = F::cast_from(0.10725146985555128001e1_f64) * t8478 * t13016;
    (t43882, t43883, t43884, t43885, t43886, t43887, t43890, t43891, t43895)
}
