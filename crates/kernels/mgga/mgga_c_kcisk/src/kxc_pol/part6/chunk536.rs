//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 536/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk536<F: Float>(t2459: F, t4998: F, t1773: F, t5005: F, t9: F, t662: F, t5014: F, t2465: F, t25: F, t1310: F, t657: F, t2464: F, t5030: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7230 = t4998 * t2459;
    let t7231 = t1773 * t7230;
    let t7233 = t9 * t5005;
    let t7234 = t7233 * t662;
    let t7242 = t5014 * t662;
    let t7253 = t25 * t2465;
    let t7254 = t1773 * t7253;
    let t7261 = t1310 * t657;
    let t7262 = t5030 * t2464;
    (t7230, t7231, t7233, t7234, t7242, t7253, t7254, t7261, t7262)
}
