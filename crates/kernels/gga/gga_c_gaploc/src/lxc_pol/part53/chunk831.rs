//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 831/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk831<F: Float>(t13870: F, t835: F, t723: F, t1457: F, t2103: F, t13900: F, t5771: F, t41136: F, t41139: F, t1445: F, t47225: F, t833: F, t47130: F, t701: F, t6066: F, t7630: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t47270 = t835 * t13870;
    let t47271 = t47270 * t723;
    let t47274 = 0.71500979903700853338e0 * t2103 * t1457 * t47271;
    let t47275 = t5771 * t13900;
    let t47280 = 0.15337170381568299871e1 * t41136;
    let t47283 = 0.76685851907841499354e0 * t41139;
    let t47286 = t833 * t1445 * t47225;
    let t47290 = 0.11502877786176224903e2 * t833 * t1445 * t47271;
    let t47294 = t47130 * t701;
    let t47296 = t7630 * t6066 * t47294;
    (t47270, t47271, t47274, t47275, t47280, t47283, t47286, t47290, t47294, t47296)
}
