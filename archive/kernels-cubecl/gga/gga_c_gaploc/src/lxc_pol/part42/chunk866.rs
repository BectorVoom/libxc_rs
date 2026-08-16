//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 866/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk866<F: Float>(t2365: F, t35623: F, t6111: F, t35445: F, t7290: F, t43581: F, t35611: F, t36762: F, t7785: F, t44712: F, t723: F) -> (F, F, F, F, F, F) {
    let t45396 = t6111 * t2365 * t35623;
    let t45397 = F::cast_from(0.29792074959875355558e-1_f64) * t45396;
    let t45407 = t6111 * t2365 * t7290 * t35445;
    let t45408 = F::cast_from(0.17875244975925213335e0_f64) * t45407;
    let t45411 = F::cast_from(0.25561950635947166451e0_f64) * t43581;
    let t45414 = t6111 * t2365 * t35611;
    let t45415 = F::cast_from(0.59584149919750711116e-1_f64) * t45414;
    let t45421 = t36762 * t7785;
    let t45423 = t44712 * t723;
    (t45397, t45408, t45411, t45415, t45421, t45423)
}
