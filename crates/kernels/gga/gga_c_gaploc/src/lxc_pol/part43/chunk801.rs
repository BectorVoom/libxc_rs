//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 801/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk801<F: Float>(t41281: F, t41283: F, t41286: F, t41290: F, t41293: F, t41295: F, t41299: F, t41312: F, t41316: F, t13016: F, t8478: F, t10867: F, t1423: F, t3247: F, t41330: F, t41337: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t43883 = 0.29792074959875355558e-1 * t41281;
    let t43884 = 0.29792074959875355558e-1 * t41283;
    let t43885 = 0.29792074959875355558e-1 * t41286;
    let t43886 = 0.29792074959875355558e-1 * t41290;
    let t43887 = 0.59584149919750711116e-1 * t41293;
    let t43888 = 0.63904876589867916127e-1 * t41295;
    let t43889 = 0.63904876589867916127e-1 * t41299;
    let t43892 = 0.63904876589867916127e-1 * t41312;
    let t43893 = 0.63904876589867916127e-1 * t41316;
    let t43895 = 0.10725146985555128001e1 * t8478 * t13016;
    let t43907 = t10867 * t1423 * t3247;
    let t43908 = 0.17875244975925213335e0 * t43907;
    let t43909 = 0.11502877786176224903e1 * t41330;
    let t43910 = 0.11916829983950142223e0 * t41337;
    (t43883, t43884, t43885, t43886, t43887, t43888, t43889, t43892, t43893, t43895, t43908, t43909, t43910)
}
