//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2521/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2521<F: Float>(t19666: F, t4806: F, t1042: F, t16208: F, t19661: F, t1065: F, t6258: F, t906: F, t5825: F, t606: F) -> (F, F, F, F, F, F, F, F) {
    let t19667 = t4806 * t19666;
    let t19668 = t1042 * t19667;
    let t19671 = t16208 * t19661;
    let t19672 = t1042 * t19671;
    let t19675 = t1065 * t6258;
    let t19676 = t19675 * t906;
    let t19677 = t1042 * t19676;
    let t19680 = t5825 * t606;
    (t19667, t19668, t19671, t19672, t19675, t19676, t19677, t19680)
}
