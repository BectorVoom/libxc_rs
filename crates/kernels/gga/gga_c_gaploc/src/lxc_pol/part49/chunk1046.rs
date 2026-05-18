//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1046/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1046<F: Float>(t33778: F, t955: F, t13064: F, t2684: F, t7354: F, t10867: F, t1423: F, t3247: F, t41330: F, t41337: F, t41340: F, t13077: F, t28439: F) -> (F, F, F, F, F, F, F) {
    let t43901 = t955 * t33778;
    let t43904 = t2684 * t7354 * t13064;
    let t43907 = t10867 * t1423 * t3247;
    let t43908 = F::new(0.17875244975925213335e0) * t43907;
    let t43909 = F::new(0.11502877786176224903e1) * t41330;
    let t43910 = F::new(0.11916829983950142223e0) * t41337;
    let t43911 = F::new(0.89376224879626066674e-1) * t41340;
    let t43912 = t13077 * t28439;
    (t43901, t43904, t43908, t43909, t43910, t43911, t43912)
}
