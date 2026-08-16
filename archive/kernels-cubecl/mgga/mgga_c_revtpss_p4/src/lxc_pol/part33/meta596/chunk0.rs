//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2015/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2015<F: Float>(t1399: F, t2434: F, t25880: F, t25899: F, t2022: F, t9646: F, t9648: F, t25875: F, t94394: F, t46361: F, t545: F, t9685: F) -> (F, F, F, F, F, F) {
    let t94633 = t2434 * t1399;
    let t94634 = t25880 * t94633;
    let t94635 = t25899 * t94634;
    let t94648 = F::cast_from(0.19637199382202157274e-3_f64) * t9646 * t2022 * t9648;
    let t94649 = t25875 * t94394;
    let t94656 = t46361 * t545;
    let t94661 = t25880 * t9685;
    (t94634, t94635, t94648, t94649, t94656, t94661)
}
