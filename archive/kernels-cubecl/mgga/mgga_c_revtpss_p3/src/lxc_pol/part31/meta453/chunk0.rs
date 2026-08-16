//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1627/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1627<F: Float>(t3172: F, t6618: F, t3711: F, t6634: F, t3610: F, t5265: F, t5293: F, t19680: F, t5302: F, t1042: F, t3153: F, t6628: F) -> (F, F, F, F, F) {
    let t20783 = t3172 * t6618;
    let t20784 = t3711 * t20783;
    let t20786 = t3172 * t6634;
    let t20787 = t3610 * t20786;
    let t20789 = t5293 * t5265;
    let t20791 = t5302 * t19680;
    let t20792 = t1042 * t20791;
    let t20795 = t6628 * t3153;
    (t20784, t20787, t20789, t20792, t20795)
}
