//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1110/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1110<F: Float>(t570: F, t6175: F, t5636: F, t1745: F, t2009: F, t1988: F, t9549: F, t1426: F, t1579: F, t2297: F, t598: F, t535: F, t8539: F) -> (F, F, F, F, F, F) {
    let t39169 = t570 * t6175;
    let t39171 = t570 * t5636;
    let t39173 = t2009 * t1745;
    let t39176 = t1988 * t9549;
    let t39182 = t598 * t1426 * t1579 * t2297;
    let t39186 = t598 * t1426 * t535 * t8539;
    (t39169, t39171, t39173, t39176, t39182, t39186)
}
