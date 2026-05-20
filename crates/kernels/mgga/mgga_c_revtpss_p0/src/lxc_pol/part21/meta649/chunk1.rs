//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2435/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2435<F: Float>(t11773: F, t11865: F, t11941: F, t11942: F, t127: F, t371: F, t11937: F, t11947: F, t3205: F, t3206: F, t676: F, t11643: F, t11994: F) -> (F, F, F, F, F) {
    let t42155 = t11865 * t11773;
    let t42170 = t11941 * t371 * t127 * t11942;
    let t42172 = t11947 * t11937;
    let t42176 = t3205 * t371 * t676 * t3206;
    let t42190 = t11994 * t11643;
    (t42155, t42170, t42172, t42176, t42190)
}
