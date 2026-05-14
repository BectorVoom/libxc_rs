//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 755/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk755<F: Float>(t13609: F, t36738: F, t13647: F, t4614: F, t813: F, t11757: F, t2714: F, t2718: F, t2365: F, t35623: F, t6111: F, t35445: F, t7290: F, t43581: F, t35611: F, t36762: F, t7785: F) -> (F, F, F, F, F, F, F, F, F) {
    let t45387 = 0.42900587942220512003e1 * t36738 * t13609;
    let t45390 = 0.61348681526273199483e1 * t813 * t4614 * t13647;
    let t45392 = 0.35750489951850426669e0 * t2714 * t11757;
    let t45394 = 0.35750489951850426669e0 * t2718 * t11757;
    let t45396 = t6111 * t2365 * t35623;
    let t45397 = 0.29792074959875355558e-1 * t45396;
    let t45407 = t6111 * t2365 * t7290 * t35445;
    let t45408 = 0.17875244975925213335e0 * t45407;
    let t45411 = 0.25561950635947166451e0 * t43581;
    let t45414 = t6111 * t2365 * t35611;
    let t45415 = 0.59584149919750711116e-1 * t45414;
    let t45421 = t36762 * t7785;
    (t45387, t45390, t45392, t45394, t45397, t45408, t45411, t45415, t45421)
}
