//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3082/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3082<F: Float>(t12254: F, t141: F, t81160: F, t43764: F, t81212: F, t3417: F, t81182: F, t1145: F, t81198: F, t81202: F, t81190: F, t81194: F) -> (F, F, F, F, F, F, F) {
    let t81439 = t141 * t12254 * t81160;
    let t81442 = t141 * t43764 * t81212;
    let t81445 = t141 * t3417 * t81182;
    let t81448 = t141 * t1145 * t81198;
    let t81451 = t141 * t1145 * t81202;
    let t81454 = t141 * t1145 * t81190;
    let t81457 = t141 * t1145 * t81194;
    (t81439, t81442, t81445, t81448, t81451, t81454, t81457)
}
