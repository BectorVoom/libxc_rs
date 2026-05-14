//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1174/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1174<F: Float>(t234: F, t5429: F, t717: F, t749: F, t410: F, t5890: F, t4982: F, t661: F, t21509: F, t61: F, t5407: F, t584: F, t5948: F, t1684: F, t1685: F, t1871: F, t5946: F) -> (F, F, F, F, F, F) {
    let t22403 = 0.69263436422725855036e2 * t234 * t717 * t5429 * t749;
    let t22404 = t410 * t5890;
    let t22406 = t4982 * t661;
    let t22409 = 0.15614757072434505372e1 * t61 * t21509;
    let t22411 = t584 * t5407 * t5948;
    let t22416 = 0.508088392e-2 * t5946 * t1684 * t1685 * t1871;
    (t22403, t22404, t22406, t22409, t22411, t22416)
}
