//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1201/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1201<F: Float>(t18426: F, t4364: F, t4366: F, t2741: F, t5980: F, t4365: F, t4424: F, t837: F, t125: F, t5966: F, t10770: F, t2652: F, t5993: F) -> (F, F, F, F, F, F) {
    let t18456 = t4364 * t18426 * t4366;
    let t18459 = t2741 * t5980;
    let t18462 = t4364 * t4365 * t4424;
    let t18466 = t4364 * t18426 * t837;
    let t18469 = t125 * t5966;
    let t18471 = t10770 * t18469 * t837;
    let t18475 = t2652 * t5993;
    (t18456, t18459, t18462, t18466, t18471, t18475)
}
