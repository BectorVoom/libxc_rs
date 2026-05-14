//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 494/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk494<F: Float>(t1233: F, t2115: F, t2119: F, t4037: F, t4054: F, t1248: F, t2075: F, t3979: F, t2133: F, t45: F, t2141: F, t4100: F, t339: F, t63: F, t67: F, t378: F, t4143: F) -> (F, F, F, F, F, F, F, F) {
    let t6035 = t2115 * t1233;
    let t6043 = t4037 * t2119;
    let t6059 = t4054 * t2119;
    let t6066 = t1248 * t3979 * t2075;
    let t6095 = t45 * t2133;
    let t6100 = t4100 * t2141;
    let t6141 = t339 * t63 * t67;
    let t6142 = t378 * t4143;
    (t6035, t6043, t6059, t6066, t6095, t6100, t6141, t6142)
}
