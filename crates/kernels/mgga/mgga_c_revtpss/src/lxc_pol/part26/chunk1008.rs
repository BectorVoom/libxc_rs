//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1008/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1008<F: Float>(t1236: F, t371: F, t676: F, t1235: F, t12627: F, t225: F, t480: F, t12629: F, t482: F, t372: F, t127: F, t3672: F) -> (F, F, F, F, F) {
    let t12984 = t371 * t676 * t1236;
    let t12985 = t1235 * t12984;
    let t12987 = t12627 * t225;
    let t12988 = t12987 * t480;
    let t12989 = t482 * t12629;
    let t12991 = t371 * t372 * t12989;
    let t12995 = t371 * t127 * t3672;
    (t12985, t12987, t12988, t12991, t12995)
}
