//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 896/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk896<F: Float>(t43881: F, t41281: F, t41283: F, t41286: F, t41290: F, t41293: F, t41305: F, t41307: F, t13016: F, t8478: F, t33778: F, t955: F, t13064: F, t2684: F, t7354: F, t10867: F, t1423: F, t3247: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t43882 = 0.76685851907841499353e0 * t43881;
    let t43883 = 0.29792074959875355558e-1 * t41281;
    let t43884 = 0.29792074959875355558e-1 * t41283;
    let t43885 = 0.29792074959875355558e-1 * t41286;
    let t43886 = 0.29792074959875355558e-1 * t41290;
    let t43887 = 0.59584149919750711116e-1 * t41293;
    let t43890 = 0.59584149919750711116e-1 * t41305;
    let t43891 = 0.89376224879626066674e-1 * t41307;
    let t43895 = 0.10725146985555128001e1 * t8478 * t13016;
    let t43901 = t955 * t33778;
    let t43904 = t2684 * t7354 * t13064;
    let t43907 = t10867 * t1423 * t3247;
    (t43882, t43883, t43884, t43885, t43886, t43887, t43890, t43891, t43895, t43901, t43904, t43907)
}
