//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 891/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk891<F: Float>(t1035: F, t1966: F, t7556: F, t30090: F, t7365: F, t1103: F, t7736: F, t1089: F, t429: F, t7553: F, t7554: F, t1998: F, t3756: F) -> (F, F, F, F, F, F) {
    let t30727 = t1035 * t1966;
    let t30728 = t30727 * t7556;
    let t30729 = F::new(0.56606566121287473723e-2) * t30728;
    let t30730 = t30090 * t7365;
    let t30769 = t7736 * t1103;
    let t30773 = t7553 * t1089 * t429 * t7554;
    let t30775 = t1998 * t3756;
    (t30727, t30729, t30730, t30769, t30773, t30775)
}
