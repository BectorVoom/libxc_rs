//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2664/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2664<F: Float>(t1063: F, t19662: F, t3172: F, t19667: F, t11994: F, t19920: F, t19692: F, t3127: F, t19650: F, t4837: F, t19929: F, t19933: F) -> (F, F, F, F, F, F, F) {
    let t65459 = t1063 * t3172 * t19662;
    let t65462 = t1063 * t3172 * t19667;
    let t65471 = t11994 * t19920;
    let t65488 = t3127 * t3172 * t19692;
    let t65493 = t4837 * t3172 * t19650;
    let t65507 = t1063 * t3172 * t19929;
    let t65510 = t1063 * t3172 * t19933;
    (t65459, t65462, t65471, t65488, t65493, t65507, t65510)
}
