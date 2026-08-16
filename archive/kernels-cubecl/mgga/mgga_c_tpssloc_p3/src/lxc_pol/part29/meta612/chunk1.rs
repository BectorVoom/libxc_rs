//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2052/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2052<F: Float>(t2240: F, t24503: F, t33: F, t39054: F, t7245: F, t50: F, t9300: F, t1240: F, t3630: F, t11588: F, t2127: F, t221: F) -> (F, F, F, F, F) {
    let t85524 = t2240 * t33 * t24503;
    let t85536 = t39054 * t7245;
    let t85539 = t50 * t9300;
    let t85628 = t1240 * t3630;
    let t85639 = t2127 * t221 * t11588;
    (t85524, t85536, t85539, t85628, t85639)
}
