//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1925/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1925<F: Float>(t27426: F, t7160: F, t1043: F, t1089: F, t7817: F, t7821: F, t1096: F, t7810: F, t988: F, t7145: F, t4820: F, t7122: F) -> (F, F, F, F, F, F, F, F) {
    let t27427 = t7160 * t27426;
    let t27433 = t7817 * t1043 * t1089;
    let t27437 = t7821 * t1043 * t1089;
    let t27440 = t7810 * t1096;
    let t27441 = t7160 * t27440;
    let t27444 = t7810 * t988;
    let t27445 = t7145 * t27444;
    let t27448 = t7122 * t4820;
    (t27427, t27433, t27437, t27440, t27441, t27444, t27445, t27448)
}
