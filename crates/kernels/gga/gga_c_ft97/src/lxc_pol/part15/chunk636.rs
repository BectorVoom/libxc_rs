//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 636/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk636<F: Float>(t20049: F, t35: F, t20022: F, t8120: F, t420: F, t419: F, t8101: F, t20031: F, t3088: F, t8088: F, t1527: F, t20039: F, t20044: F, t423: F, t11299: F, t15840: F, t15855: F, t15866: F, t8079: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t20050 = t20049 * t35;
    let t20065 = t8120 * t20022;
    let t20066 = t420 * t20065;
    let t20067 = t419 * t20066;
    let t20069 = t8101 * t20022;
    let t20070 = t420 * t20069;
    let t20071 = t419 * t20070;
    let t20073 = t3088 * t20031;
    let t20074 = t419 * t20073;
    let t20076 = t8088 * t20022;
    let t20077 = t420 * t20076;
    let t20078 = t419 * t20077;
    let t20080 = t1527 * t20039;
    let t20081 = t419 * t20080;
    let t20083 = t423 * t20044;
    let t20084 = t420 * t20083;
    let t20085 = t419 * t20084;
    let t20087 = t8079 - 0.42562405586419753086e-2 * t11299 + 0.85124811172839506172e-2 * t15840 - 0.12768721675925925926e-1 * t15855 + 0.63843608379629629629e-2 * t15866 + 0.19862455940329218107e-1 * t20067 - 0.51074886703703703704e-1 * t20071 + 0.25537443351851851852e-1 * t20074 + 0.38306165027777777778e-1 * t20078 - 0.38306165027777777778e-1 * t20081 + 0.6384360837962962963e-2 * t20085;
    (t20050, t20065, t20067, t20069, t20071, t20074, t20076, t20078, t20081, t20083, t20085, t20087)
}
