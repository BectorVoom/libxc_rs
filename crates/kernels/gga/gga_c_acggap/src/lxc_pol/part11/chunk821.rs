//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 821/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk821<F: Float>(t1117: F, t7329: F, t1103: F, t7736: F, t1089: F, t429: F, t7553: F, t7554: F, t1998: F, t3756: F, t3761: F, t141: F, t167: F, t2035: F, t1096: F, t1992: F) -> (F, F, F, F, F, F, F, F) {
    let t30767 = t7329 * t1117;
    let t30769 = t7736 * t1103;
    let t30773 = t7553 * t1089 * t429 * t7554;
    let t30775 = t1998 * t3756;
    let t30777 = t1998 * t3761;
    let t30779 = t167 * t141;
    let t30780 = t2035 * t30779;
    let t30781 = t1992 * t1096;
    (t30767, t30769, t30773, t30775, t30777, t30779, t30780, t30781)
}
