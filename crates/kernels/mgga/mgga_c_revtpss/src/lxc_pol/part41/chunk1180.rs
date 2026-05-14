//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1180/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1180<F: Float>(t19380: F, t373: F, t371: F, t372: F, t19463: F, t366: F, t3094: F, t4186: F, t4781: F, t3092: F, t4786: F, t6092: F, t11703: F, t11710: F, t6267: F, t3091: F) -> (F, F, F, F, F) {
    let t19768 = t373 * t19380;
    let t19770 = t371 * t372 * t19768;
    let t19773 = t19463 * t366;
    let t19776 = t3094 * t4186;
    let t19777 = t4781 * t19776;
    let t19778 = t3092 * t19777;
    let t19781 = t6092 * t4786;
    let t19782 = t11703 * t19781;
    let t19785 = t11710 * t6267;
    let t19786 = t3091 * t19785;
    (t19770, t19773, t19778, t19782, t19786)
}
